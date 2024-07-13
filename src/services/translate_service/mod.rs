
pub mod translate_text;
pub mod list_languages;
pub mod detect_language;


use crate::auth::service_account::ServiceAccountCredentials;
use super::ServiceBase;


static TRANSLATE_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/cloud-translation";
static TRANSLATE_SERVICE_BASE_URL: &str = "https://translation.googleapis.com/language/translate";

#[derive(Debug, Clone)]
pub struct TranslateService {
    base: ServiceBase
}


impl TranslateService {
    /// Create `TranslateService` Authenticate by using API keys.
    ///
    /// * `api_key` -  API key to use to authenticate to Google Cloud APIs and services that support API keys.
    pub fn new_with_api_key(api_key: String) -> Self {
        return Self { base: ServiceBase::new_with_api_key(api_key) }
    }

    /// Create `TranslateService` Authenticate by using API keys.
    ///
    /// * `service_account_credentials` -  `ServiceAccountCredentials` to use to authenticate to Google Cloud APIs.
    pub fn new_with_credentials(service_account_credentials: ServiceAccountCredentials) -> Self {
        return Self { base: ServiceBase::new_with_credentials(service_account_credentials, vec![TRANSLATE_SERVICE_SCOPE]) }
    }
}

enum TranslateServiceV2Type {
    Translate,
    Detect,
    Languages
}

impl TranslateServiceV2Type {
    fn path(&self) -> &str {
        match *self {
            TranslateServiceV2Type::Translate => "",
            TranslateServiceV2Type::Detect => "detect",
            TranslateServiceV2Type::Languages =>  "languages"
        }
    }
}