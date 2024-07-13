
use serde::{Deserialize, Serialize};

use crate::services::route_service::common_models::{FallbackInfo, RouteLocalizedValues, RouteTravelAdvisory, Status};


/// The Response body of ComputeRouteMatrixRequest.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRouteMatrix
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeRouteMatrixResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<RouteMatrixElementCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_advisory: Option<RouteTravelAdvisory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_info: Option<FallbackInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_values: Option<RouteLocalizedValues>,

    pub origin_index: i32,
    pub destination_index: i32,

}


/// The condition of the route being returned.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRouteMatrix
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RouteMatrixElementCondition {
    RouteMatrixElementConditionUnspecified,
    RouteExists,
    RouteNotFound,

}