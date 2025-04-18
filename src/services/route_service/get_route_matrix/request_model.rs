
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::{Ok, Result};
use serde_json::Value;

use crate::services::route_service::common_models::{ExtraComputation, RouteModifiers, RouteTravelMode, RoutingPreference, TrafficModel, TransitPreferences, Unit, WayPoint};




#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeRouteMatrixRequest {
    // required fields

    /// Several size restrictions apply to the cardinality of origins and destinations <br>
    /// * The sum of the number of origins + the number of destinations specified as either placeId or address must be no greater than 50.
    /// * The product of number of origins × number of destinations must be no greater than 625 in any case.
    /// * The product of the number of origins × number of destinations must be no greater than 100 if routingPreference is set to TRAFFIC_AWARE_OPTIMAL.
    /// * The product of the number of origins × number of destinations must be no greater than 100 if travelMode is set to TRANSIT.
    origins: Vec<RouteMatrixOrigin>,
    destinations: Vec<RouteMatrixOrigin>,

    // optional fields
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    params: Option<ComputeRouteMatixRequestOptinalParams>
}


impl ComputeRouteMatrixRequest {
    pub fn new(origin: &Vec<RouteMatrixOrigin>, destination: &Vec<RouteMatrixOrigin>, params: Option<HashMap<String, Value>>) -> Result<Self> {

        let additional_params: Option<ComputeRouteMatixRequestOptinalParams> = if let Some(params) = params {
            let additional_params_string = serde_json::to_string(&params)?;
            serde_json::from_str(&additional_params_string)?
        } else {
            None
        };

        return Ok(Self{
            origins: origin.to_owned(),
            destinations: destination.to_owned(),
            params: additional_params
        });
    }
}


/// Optional fields for computing routes.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeRouteMatixRequestOptinalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    travel_mode: Option<RouteTravelMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_preference: Option<RoutingPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    departure_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arrival_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_computations: Option<Vec<ExtraComputation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_model: Option<TrafficModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_preferences: Option<TransitPreferences>
}




/// A single origin for ComputeRouteMatrixRequest.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRouteMatrix
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMatrixOrigin {
    waypoint: WayPoint,

    #[serde(skip_serializing_if = "Option::is_none")]
    route_modifiers: Option<RouteModifiers>
}

impl RouteMatrixOrigin {
    pub fn new_with_modifiers(waypoint: &WayPoint, route_modifiers: RouteModifiers) -> Self {
        Self {
            waypoint: waypoint.to_owned(),
            route_modifiers: Some(route_modifiers)
        }
    }

    pub fn new(waypoint: &WayPoint) -> Self {
        Self {
            waypoint: waypoint.to_owned(),
            route_modifiers: None
        }
    }
}