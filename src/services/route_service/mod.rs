

use crate::auth::service_account::ServiceAccountCredentials;
use super::ServiceBase;

pub mod get_route;
pub mod get_route_matrix;
pub mod common_models;

static ROUTE_SERVICE_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
static GET_ROUTE_URL: &str = "https://routes.googleapis.com/directions/v2:computeRoutes";
static GET_ROUTE_MATRIX_URL: &str = "https://routes.googleapis.com/distanceMatrix/v2:computeRouteMatrix";

#[derive(Debug, Clone)]
pub struct RouteService {
    base: ServiceBase
}


impl RouteService {
    /// Create `RouteService` Authenticate by using API keys.
    ///
    /// * `api_key` -  API key to use to authenticate to Google Cloud APIs and services that support API keys.
    pub fn new_with_api_key(api_key: String) -> Self {
        return Self { base: ServiceBase::new_with_api_key(api_key) }
    }

    /// Create `RouteService` Authenticate by using API keys.
    ///
    /// * `service_account_credentials` -  `ServiceAccountCredentials` to use to authenticate to Google Cloud APIs.
    pub fn new_with_credentials(service_account_credentials: ServiceAccountCredentials) -> Self {
        return Self { base: ServiceBase::new_with_credentials(service_account_credentials, vec![ROUTE_SERVICE_SCOPE]) }
    }
}