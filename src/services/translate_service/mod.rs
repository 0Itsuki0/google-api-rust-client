
pub mod translate;
use crate::auth::service_account::ServiceAccountCredentials;


static TRANSLATE_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/cloud-translation";
static TRANSLATE_SERVICE_BASE_URL: &str = "https://translation.googleapis.com/language/translate";

#[derive(Debug, Clone)]
pub struct TranslateService {
    api_key: Option<String>,
    service_account_credentials: Option<ServiceAccountCredentials>, 
}


impl TranslateService {
    pub fn new_with_api_key(api_key: String) -> Self {
        return Self { api_key: Some(api_key), service_account_credentials: None }
    }

    pub fn new_with_credentials(service_account_credentials: ServiceAccountCredentials) -> Self {
        let scoped_credentials = service_account_credentials.with_scopes(vec![TRANSLATE_SERVICE_SCOPE]);
        return Self { api_key: None, service_account_credentials: Some(scoped_credentials) }
    }
}


enum BasicServiceType {
    Translate, 
    // Detect, 
    // Languages
}

impl BasicServiceType {
    fn path(&self) -> &str {
        match *self {
            BasicServiceType::Translate => "",
            // ServiceType::Detect => "detect",
            // ServiceType::Languages =>  "languages"
        }
    }
}