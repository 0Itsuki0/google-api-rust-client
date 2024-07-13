

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::{Ok, Result};
use serde_json::Value;

use crate::services::route_service::common_models::{ExtraComputation, RouteModifiers, RouteTravelMode, RoutingPreference, TrafficModel, TransitPreferences, Unit, WayPoint};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeRouteRequest {
    // required fields
    origin: WayPoint,
    destination: WayPoint,

    // optional fields
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    params: Option<ComputeRouteRequestOptinalParams>
}


impl ComputeRouteRequest {
    pub fn new(origin: &WayPoint, destination: &WayPoint, params: Option<HashMap<String, Value>>) -> Result<Self> {
        println!("request parameter {:?}", params);
        let additional_params: Option<ComputeRouteRequestOptinalParams> = if let Some(params) = params {
            let additional_params_string = serde_json::to_string(&params)?;
            // println!("{:?}", additional_params_string);
            serde_json::from_str(&additional_params_string)?
        } else {
            None
        };
        // println!("{:?}", additional_params);

        return Ok(Self{
            origin: origin.to_owned(),
            destination: destination.to_owned(),
            params: additional_params
        });
    }
}



/// Optional fields for computing routes.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeRouteRequestOptinalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediates: Option<Vec<WayPoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    travel_mode: Option<RouteTravelMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_preference: Option<RoutingPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polyline_quality: Option<PolyLineQuality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polyline_encoding: Option<PolylineEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    departure_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arrival_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_alternative_routes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_modifiers: Option<RouteModifiers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    optimize_waypoint_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_reference_routes: Option<Vec<ReferenceRoute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_computations: Option<Vec<ExtraComputation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_model: Option<TrafficModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transit_preferences: Option<TransitPreferences>
}




/// A set of values that specify the quality of the polyline.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/RoutingPreference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolyLineQuality {
    PolylineQualityUnspecified,
    HighQuality,
    Overview,
}

/// Specifies the preferred type of polyline to be returned.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolylineEncoding {
    PolylineEncodingUnspecified,
    EncodedPolyline,
    GeoJsonLinestring,
}



/// A supported reference route on the ComputeRoutesRequest.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReferenceRoute {
    ReferenceRouteUnspecified,
    FuelEfficient,
}
