use super::{BasicServiceType, TranslateService, TRANSLATE_SERVICE_BASE_URL};

use serde::{Serialize, Deserialize};
use anyhow::Result;
use reqwest::{Client, Url};

impl TranslateService {

    /// Translates text into the target language
    /// with default format, automatically detected source language, using nmt Neural Machine Translation (NMT) model. <br>
    /// See https://g.co/cloud/translate/v2/translate-reference#supported_languages
    /// https://cloud.google.com/translate/docs/reference/rest/v2/translate
    /// https://cloud.google.com/translate/docs/basic/translating-text#translate_translate_text-drest
    ///
    /// * `text` -  an array of strings to be translated.
    /// * `target` - The language to use for translation of the input text.
    pub async fn translate(&mut self, text: Vec<&str>, target: &str) -> Result<TranslateTextResponse>{
        let request_body = TranslateTextRequest::new(text, target);
        self.post_translate_request(request_body).await
    }

    /// Translates text into the target language with custom parameters. <br>
    /// See https://g.co/cloud/translate/v2/translate-reference#supported_languages
    /// https://cloud.google.com/translate/docs/reference/rest/v2/translate
    /// https://cloud.google.com/translate/docs/basic/translating-text#translate_translate_text-drest
    ///
    /// * `text` -  an array of strings to be translated.
    /// * `target` - The language to use for translation of the input text.
    /// * `format` - The format of the source text, in either HTML (default) or plain-text. A value of html indicates HTML and a value of text indicates plain-text.
    /// * `source` - The language of the source text.
    /// * `model` - The translation model. Cloud Translation - Basic offers only the nmt Neural Machine Translation (NMT) model. If the model is base, the request is translated by using the NMT model..
    pub async fn translate_with_params(&mut self, text: Vec<&str>, target: &str, format: Option<&str>, source: Option<&str>, model: Option<&str>) -> Result<TranslateTextResponse>{
        let request_body = TranslateTextRequest::new_with_params(text, target, format, source, model);
        self.post_translate_request(request_body).await

    }

    async fn post_translate_request(&mut self, request_body: TranslateTextRequest) -> Result<TranslateTextResponse> {

        let base_url = Url::parse(&format!("{}/v2/{}", TRANSLATE_SERVICE_BASE_URL, BasicServiceType::Translate.path()))?;
        let headers = self.base.create_headers().await?;
        let builder = Client::new().post(base_url)
                .headers(headers)
                .body(serde_json::to_string(&request_body)?);

        let body = self.base.make_request(builder).await?;

        Ok(serde_json::from_str::<TranslateTextResponse>(&body)?)

    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextRequest {
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
    fn new(text: Vec<&str>, target: &str) -> Self {
        return Self{
            q: text.into_iter().map(|s| s.to_owned()).collect(),
            target: target.to_owned(),
            format: None,
            source: None,
            model: None
        };
    }

    fn new_with_params(text: Vec<&str>, target: &str, format: Option<&str>, source: Option<&str>, model: Option<&str>) -> Self {
        return Self{
            q: text.into_iter().map(|s| s.to_owned()).collect(),
            target: target.to_owned(),
            format: format.map(String::from),
            source: source.map(String::from),
            model: model.map(String::from)
        };
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponse {
    pub data: TranslateTextResponseData
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponseData {
    pub translations: Vec<TranslateTextResponseTranslation>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponseTranslation {
    pub translated_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_source_language: Option<String>,
}
