pub mod translate_service;
pub mod route_service;
pub mod service_error;

use anyhow::{bail, Result};
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, RequestBuilder};
use service_error::ServiceErrorResponse;

use crate::auth::service_account::ServiceAccountCredentials;


#[derive(Debug, Clone)]
struct ServiceBase {
    api_key: Option<String>,
    service_account_credentials: Option<ServiceAccountCredentials>,
}

impl ServiceBase {
    fn new_with_api_key(api_key: String) -> Self {
        return Self { api_key: Some(api_key), service_account_credentials: None }
    }

    fn new_with_credentials(service_account_credentials: ServiceAccountCredentials, scopes: Vec<&str>) -> Self {
        let scoped_credentials = service_account_credentials.with_scopes(scopes);
        return Self { api_key: None, service_account_credentials: Some(scoped_credentials) }
    }
}


impl ServiceBase {

    async fn create_headers(&mut self) -> Result<HeaderMap>{
        let mut headers = HeaderMap::new();

        if let Some(api_key) = &self.api_key {
            headers.insert("X-goog-api-key", HeaderValue::from_str(&api_key)?);
        } else if let Some(mut credentials) = self.service_account_credentials.to_owned() {
            let token = credentials.get_access_token().await?;
            self.service_account_credentials = Some(credentials);
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token))?);
        } else {
            bail!("Unknown Auth Method!")
        };

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"));
        Ok(headers)
    }

    async fn make_request(&mut self, request: RequestBuilder) -> Result<String> {
        let response = request.send().await?;
        let status_code = response.status();
        let body: String = response.text().await?;
        println!("{}", body);
        if !status_code.is_success() {
            let error_response: ServiceErrorResponse = serde_json::from_str(&body).unwrap_or_default();
            bail!(format!("Response Error! Code: {}, Message: {}", error_response.error.code, error_response.error.message));
        }

        Ok(body)
    }
}
