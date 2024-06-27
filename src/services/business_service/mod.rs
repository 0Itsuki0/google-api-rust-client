pub mod business;
use crate::auth::service_account::ServiceAccountCredentials;

static MY_BUSINESS_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/plus.business.manage";

pub struct BusinessService {
    api_key: Option<String>,
    service_account_credentials: Option<ServiceAccountCredentials>,
}

impl BusinessService {
    pub fn new_with_api_key(api_key: String) -> Self {
        return Self {
            api_key: Some(api_key),
            service_account_credentials: None,
        };
    }

    pub fn new_with_credentials(service_account_credentials: ServiceAccountCredentials) -> Self {
        let scoped_credentials =
            service_account_credentials.with_scopes(vec![MY_BUSINESS_SERVICE_SCOPE]);
        return Self {
            api_key: None,
            service_account_credentials: Some(scoped_credentials),
        };
    }
}
