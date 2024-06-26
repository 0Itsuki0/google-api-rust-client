
use super::{BasicServiceType, TranslateService, TRANSLATE_SERVICE_BASE_URL};

use serde::{Serialize, Deserialize};
use anyhow::Result;
use reqwest::{Client, Url};

impl TranslateService {


    /// Detects the language of texts. <br>
    /// See https://cloud.google.com/translate/docs/basic/detecting-language
    ///
    /// * `text` -  an array of strings to upon which to perform language detection.
    pub async fn detect_language(&mut self, text: Vec<&str>) -> Result<DetectLanguageResponse>{

        let base_url = Url::parse(&format!("{}/v2/{}", TRANSLATE_SERVICE_BASE_URL, BasicServiceType::Detect.path()))?;
        let headers = self.base.create_headers().await?;
        let request_body = DetectLanguageRequest::new(text);
        let builder = Client::new().post(base_url)
                .headers(headers)
                .body(serde_json::to_string(&request_body)?);

        let body = self.base.make_request(builder).await?;

        Ok(serde_json::from_str::<DetectLanguageResponse>(&body)?)

    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetectLanguageRequest {
    q: Vec<String>
}

impl DetectLanguageRequest {
    fn new(text: Vec<&str>) -> Self {
        return Self{
            q: text.into_iter().map(|s| s.to_owned()).collect(),
        };
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectLanguageResponse {
    pub data: DetectLanguageResponseList
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectLanguageResponseList {
    pub detections: Vec<Vec<DetectLanguageResponseListValue>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectLanguageResponseListValue {
    pub language: String
}