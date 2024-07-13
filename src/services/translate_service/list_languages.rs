
use super::{TranslateServiceV2Type, TranslateService, TRANSLATE_SERVICE_BASE_URL};

use serde::{Serialize, Deserialize};
use anyhow::Result;
use reqwest::{Client, Url};

impl TranslateService {


    /// List supported languages. <br>
    /// See https://cloud.google.com/translate/docs/basic/discovering-supported-languages
    ///
    /// * `target` - The target language code for the results.
    /// If specified, then the language names are returned in the name field of the response, localized in the target language.
    /// If you do not supply a target language, then the name field is omitted from the response and only the language codes are returned.
    ///  * `model` - The supported languages for a particular translation model.
    /// For Cloud Translation - Basic, the value can be nmt to return languages supported by the Neural Machine Translation (NMT) model.
    pub async fn list_languages(&mut self, target: Option<&str>, model: Option<&str>) -> Result<ListLanguageResponse>{

        let base_url = Url::parse(&format!("{}/v2/{}", TRANSLATE_SERVICE_BASE_URL, TranslateServiceV2Type::Languages.path()))?;
        let headers = self.base.create_headers().await?;
        let request_query = ListLanguageRequest::new(target, model);

        let builder = Client::new().get(base_url)
                .query(&request_query)
                .headers(headers);

        let body = self.base.make_request(builder).await?;


        Ok(serde_json::from_str::<ListLanguageResponse>(&body)?)

    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListLanguageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>, }

impl ListLanguageRequest {
    fn new(target: Option<&str>, model: Option<&str>) -> Self {
        return Self{
            target: target.map(String::from),
            model: model.map(String::from),
        };
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLanguageResponse {
    pub data: GetSupportedLanguagesResponseList
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSupportedLanguagesResponseList {
    pub languages: Vec<GetSupportedLanguagesResponseLanguage>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSupportedLanguagesResponseLanguage {
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
