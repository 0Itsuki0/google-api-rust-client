
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::route_service::common_models::{FallbackInfo, RouteLocalizedValues, RouteTravelAdvisory, SpeedReadingInterval, Status, TollInfo};

use super::super::common_models::{LatLng, LocalizedText, Location, RouteTravelMode};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeRouteResponse {
    pub routes: Vec<Route>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_info: Option<FallbackInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geocoding_results: Option<GeocodingResults>
}


/// Contains a route, which consists of a series of connected road segments that join beginning, ending, and intermediate waypoints.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_labels: Option<Vec<RouteLabel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<RouteLeg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<Polyline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_advisory: Option<RouteTravelAdvisory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_intermediate_waypoint_index: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_values: Option<RouteLocalizedValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_token: Option<String>,
}

/// Labels for the Route that are useful to identify specific properties of the route to compare against others.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RouteLabel {
    RouteLabelUnspecified,
    DefaultRoute,
    DefaultRouteAlternate,
    FuelEfficient
}


/// Contains a segment between non-via waypoints.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLeg {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<Polyline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<RouteLegStep>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_advisory: Option<RouteLegTravelAdvisory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_values: Option<RouteLegLocalizedValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_overview: Option<StepsOverview>
}


/// Encapsulates an encoded polyline.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Polyline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_polyline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_json_linestring: Option<Value>
}


/// Contains a segment of a RouteLeg. A step corresponds to a single navigation instruction. Route legs are made up of steps.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLegStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<Polyline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation_instruction: Option<NavigationInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_advisory: Option<RouteLegStepTravelAdvisory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_values: Option<RouteLegStepLocalizedValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_details: Option<RouteLegStepTransitDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_mode: Option<RouteTravelMode>
}


/// Encapsulates navigation instructions for a RouteLegStep.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationInstruction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maneuver: Option<Maneuver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>
}

/// A set of values that specify the navigation action to take for the current step (for example, turn left, merge, or straight).
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Maneuver {
    ManeuverUnspecified,
    TurnSlightLeft,
    TurnSharpLeft,
    UturnLeft,
    TurnLeft,
    TurnSlightRight,
    TurnSharpRight,
    UturnRight,
    TurnRight,
    Straight,
    RampLeft,
    RampRight,
    Merge,
    ForkLeft,
    ForkRight,
    Ferry,
    FerryTrain	,
    RoundaboutLeft,
    RoundaboutRight,
    Depart,
    NameChange
}

/// Contains the additional information that the user should be informed about, such as possible traffic zone restrictions on a leg step.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLegStepTravelAdvisory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_reading_intervals: Option<Vec<SpeedReadingInterval>>,
}


/// Text representations of certain properties.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLegStepLocalizedValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<LocalizedText>,
}






/// Additional information for the RouteLegStep related to TRANSIT routes.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLegStepTransitDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_details: Option<TransitStopDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localized_values: Option<TransitDetailsLocalizedValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headsign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_line: Option<TransitLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trip_short_text: Option<String>,
}



/// Details about the transit stops for the RouteLegStep.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitStopDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_stop: Option<TransitStop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_stop: Option<TransitStop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,
}

/// Information about a transit stop.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitStop {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>
}



/// Localized descriptions of values for RouteTransitDetails.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitDetailsLocalizedValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_time: Option<LocalizedTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<LocalizedTime>,
}

/// Localized description of time.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedTime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// Contains information about the transit line used in this step.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agencies: Option<Vec<TransitAgency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<TransitVehicle>

}

/// A transit agency that operates a transit line.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitAgency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Information about a vehicle used in transit routes.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitVehicle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransitVehicleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_icon_uri: Option<String>,
}


/// The type of vehicles for transit routes..
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransitVehicleType {
    TransitVehicleTypeUnspecified,
    Bus,
    CableCar,
    CommuterTrain,
    Ferry,
    Funicular,
    GondolaLift,
    HeavyRail,
    HighSpeedTrain,
    IntercityBus,
    LongDistanceTrain,
    MetroRail,
    Monorail,
    Other,
    Rail,
    ShareTaxi,
    Subway,
    Tram,
    Trolleybus
}



/// Contains the additional information that the user should be informed about on a leg step, such as possible traffic zone restrictions.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLegTravelAdvisory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toll_info: Option<TollInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_reading_intervals: Option<Vec<SpeedReadingInterval>>
}





/// Text representations of certain properties.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLegLocalizedValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<LocalizedText>,
}



/// Provides overview information about a list of RouteLegStep.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepsOverview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_modal_segments: Option<Vec<MultiModalSegment>>
}


/// Provides summarized information about different multi-modal segments of the RouteLeg.steps.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiModalSegment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation_instruction: Option<NavigationInstruction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_mode: Option<RouteTravelMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_start_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_end_index: Option<i32>,
}

/// A latitude-longitude viewport, represented as two diagonally opposite low and high points.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    low: LatLng,
    high: LatLng
}




/// Contains GeocodedWaypoints for origin, destination and intermediate waypoints.
/// Only populated for address waypoints.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeocodingResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<GeocodedWaypoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<GeocodedWaypoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<Vec<GeocodedWaypoint>>,
}


/// Details about the locations used as waypoints.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeocodedWaypoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geocoder_status: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_match: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_waypoint_request_index: Option<u32>
}
