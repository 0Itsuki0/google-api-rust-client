use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Ok, Result};
use chrono::{Local, Duration};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::header::{HeaderValue, CONTENT_TYPE, HeaderMap};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::auth_error::AuthErrorResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccountCredentials {
    r#type: String,
    project_id: String, 
    private_key_id: String, 
    private_key: String,
    client_email: String, 
    client_id: String,
    auth_uri: String, 
    token_uri: String, 
    auth_provider_x509_cert_url: String, 
    client_x509_cert_url: String, 
    universe_domain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<Token>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<Vec<String>>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    expiration_time: i64,
    access_token: String,
}

impl ServiceAccountCredentials {
    pub fn from_service_account_file(filepath: PathBuf) -> Result<Self> {
        let credentials_json = fs::read_to_string(filepath)?;
        Ok(serde_json::from_str::<ServiceAccountCredentials>(&credentials_json)?)
    }

    pub fn from_service_account_info(credentials_json: String) -> Result<Self>  {
        Ok(serde_json::from_str::<ServiceAccountCredentials>(&credentials_json)?)
    }

    pub fn with_scopes(&self, scopes: Vec<&str>) -> Self {
        let mut scoped_credentials = self.clone();
        scoped_credentials.scopes = Some(scopes.into_iter().map(|s| s.to_owned()).collect());
        return scoped_credentials
    }

    pub fn with_subject(&self, subject: &str) -> Self {
        let mut subjected_credential = self.clone();
        subjected_credential.sub = Some(subject.to_owned());
        return subjected_credential
    }

    pub async fn get_access_token(&mut self) -> Result<String> {
        let now = Local::now();
        let iat = now.timestamp();

        match self.token.clone() {
            Some(token) => {
                if iat > token.expiration_time {
                    let jwt = self.make_assertion()?;
                    let access_token = self.request_token(&jwt).await?;
                    self.token = Some(Token{
                        expiration_time: (now + Duration::minutes(58)).timestamp(),
                        access_token: access_token.clone(),
                    });
                    return Ok(access_token);
                } else {
                    return Ok(token.access_token.clone());
                }
            },
            None => {
                let jwt = self.make_assertion()?;
                let access_token = self.request_token(&jwt).await?;
                self.token = Some(Token{
                    expiration_time: (now + Duration::minutes(58)).timestamp(),
                    access_token: access_token.clone(),
                });
                return Ok(access_token);
            }
        };
    }

    fn make_assertion(&self) -> Result<String> {
        let scope: String = match self.scopes.clone() {
            Some(scopes) => {
                scopes.join(",")
            },
            None => {
                "".to_owned()
            },
        };
    
        let mut header = Header::new(Algorithm::RS256);
        header.typ = Some("JWT".to_owned());
        header.kid = Some("".to_owned());
        
        let now = Local::now();
        let iat = now.timestamp();
        let exp = (now + Duration::hours(1)).timestamp();
        let claims = Claims {
            iss: self.client_id.clone(),
            sub: self.sub.clone(),
            aud: self.token_uri.clone(),
            scope,
            iat,
            exp,
        };
    
        let jwt = encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes())?,
        )?;
    
        return Ok(jwt);
    }


    async fn request_token(&self, assertion: &str) -> Result<String> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
    
        let grant_type = "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned();
    
        let body_encoded = url_encoded_data::stringify(&[
            ("assertion", assertion),
            ("grant_type", &grant_type)
        ]);
    
        let response = client
            .post(self.token_uri.clone())
            .headers(headers.clone())
            .body(body_encoded)
            .send()
            .await?;
    
        let status_code = response.status();
        let body: String = response.text().await?;

        if !status_code.is_success() {
            let error_response: AuthErrorResponse = serde_json::from_str(&body).unwrap_or_default();
            bail!(format!("Response Error: {}! Message: {}", error_response.error, error_response.error_description));
        }

        let v: Value = serde_json::from_str(&body)?;
        if let Some(access_token) = v["access_token"].as_str() {
            return Ok(access_token.to_owned());
        } else {
            bail!("Error parsing for access token!")
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
    aud: String,
    scope: String,
    iat: i64,
    exp: i64,
}
