use super::{BasicServiceType, TranslateService};

use serde::{Serialize, Deserialize};
use crate::services::service_error::ServiceErrorResponse;

use anyhow::{bail, Result};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{header::HeaderMap, Client, Url};

// Translates text into the target language.
// Target must be an ISO 639-1 language code.
// See https://g.co/cloud/translate/v2/translate-reference#supported_languages
// https://cloud.google.com/translate/docs/reference/rest/v2/translate
// https://cloud.google.com/translate/docs/basic/translating-text#translate_translate_text-drest

impl TranslateService {

    pub async fn translate(self, text: Vec<&str>, target: &str) -> Result<TranslateTextResponse>{
        let base_url = Url::parse(&format!("https://translation.googleapis.com/language/translate/v2/{}", BasicServiceType::Translate.path()))?;
        let mut headers = HeaderMap::new();
        

        if let Some(api_key) = self.api_key {
            headers.insert("X-goog-api-key", HeaderValue::from_str(&api_key)?);
        } else if let Some(mut credentials) = self.service_account_credentials {
            let token = credentials.get_access_token().await?;
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token))?);
        } else {
            bail!("Unknown Auth Method!")
        };

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"));

        let request_body = TranslateTextRequest::new(text, target);

        let response = Client::new().post(base_url)
        .headers(headers)
        .body(serde_json::to_string(&request_body)?)
        .send()
        .await?;

        let status_code = response.status();
        let body: String = response.text().await?;

        // println!("{:?}", body);
        if !status_code.is_success() {
            let error_response: ServiceErrorResponse = serde_json::from_str(&body).unwrap_or_default();
            bail!(format!("Response Error! Code: {}, Message: {}", error_response.error.code, error_response.error.message));
        }

        Ok(serde_json::from_str::<TranslateTextResponse>(&body)?)

    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextRequest {
    q: Vec<String>,
    target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>
}

impl TranslateTextRequest {
    pub fn new(text: Vec<&str>, target: &str) -> Self {
        return Self{
            q: text.into_iter().map(|s| s.to_owned()).collect(), 
            target: target.to_owned(), 
            format: None,
            source: None,
            model: None
        };
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponse {
    data: TranslateTextResponseData
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponseData {
    translations: Vec<TranslateTextResponseTranslation>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponseTranslation {
    translated_text: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detected_source_language: Option<String>,
}
