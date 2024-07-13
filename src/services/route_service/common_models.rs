
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use anyhow::{Ok, Result};
use serde_json::Value;

/// Encapsulates a waypoint.
/// Waypoints mark both the beginning and end of a route, and include intermediate stops along the route. <br>
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/Waypoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WayPoint {
    // Exactly one of the following need to be present
    // Union field location_type can be only one of the following:
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    // End of list of possible types for union field location_type.

    // optional fields
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    params: Option<WayPointOptionalParams>
}


/// Optional parameters for WayPoint
/// * `via`: Marks this waypoint as a milestone rather a stopping point.
/// * `vehicle_stopover`: Indicates that the waypoint is meant for vehicles to stop at, where the intention is to either pickup or drop-off.
/// * `side_of_road`: Indicates that the location of this waypoint is meant to have a preference for the vehicle to stop at a particular side of road.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WayPointOptionalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    via: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vehicle_stopover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side_of_road: Option<bool>
}

impl WayPoint {
    pub fn new_from_location(location: Location, params: Option<HashMap<String, Value>>) -> Result<Self> {
        let additional_params = Self::get_additional_params(params)?;
        Ok(Self{
            location: Some(location),
            place_id: None,
            address: None,
            params: additional_params
        })
    }

    pub fn new_from_place_id(place_id: &str, params: Option<HashMap<String, Value>>) -> Result<Self> {
        let additional_params = Self::get_additional_params(params)?;
        Ok(Self{
            location: None,
            place_id: Some(place_id.to_owned()),
            address: None,
            params: additional_params
        })
    }

    pub fn new_from_address(address: &str, params: Option<HashMap<String, Value>>) -> Result<Self> {
        let additional_params = Self::get_additional_params(params)?;
        Ok(Self{
            location: None,
            place_id: None,
            address: Some(address.to_owned()),
            params: additional_params
        })
    }

    fn get_additional_params(params: Option<HashMap<String, Value>>) -> Result<Option<WayPointOptionalParams>> {
        let additional_params: Option<WayPointOptionalParams> = if let Some(params) = params {
            let additional_params_string = serde_json::to_string(&params)?;
            serde_json::from_str(&additional_params_string)?
        } else {
            None
        };
        Ok(additional_params)
    }

}

/// Encapsulates a location (a geographic point, and an optional heading). <br>
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/Location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub lat_lng: LatLng,
    /// Heading values can be from 0 to 360,
    /// where 0 specifies a heading of due North, 90 specifies a heading of due East, and so on.
    /// You can use this field only for DRIVE and TWO_WHEELER RouteTravelMode.
    pub heading: Option<u32>
}

impl Location {
    /// Create a Location at a geographic point, and an optional heading
    pub fn new(latitude: f32, longitude: f32, heading: Option<u32>) -> Self {
        Self {
            lat_lng: LatLng { latitude, longitude },
            heading
        }
    }
}


/// An object that represents a latitude/longitude pair. <br>
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/LatLng
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatLng {
    pub latitude: f32,
    pub longitude: f32,
}



/// A set of values used to specify the mode of travel.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/RouteTravelMode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RouteTravelMode {
    TravelModeUnspecified,
    Drive,
    Bycycle,
    Walk,
    TwoWheeler,
    Transit
}


/// Localized variant of a text in a particular language.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/LocalizedText
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>
}



/// A set of optional conditions to satisfy when calculating the routes.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/RouteModifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteModifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_tolls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_highways: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_ferries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_indoor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vehicle_info: Option<VehicleInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toll_passes: Option<Vec<String>>
}


impl RouteModifiers {
    /// A set of optional conditions to satisfy when calculating the routes.
    /// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/RouteModifiers
    pub fn new(avoid_tolls: Option<bool>, avoid_highways: Option<bool>, avoid_ferries: Option<bool>, avoid_indoor: Option<bool>, emission_type: Option<EmissionType>, toll_passes: Option<Vec<String>>) -> Self{
        let vehicle_info:Option<VehicleInfo>= if let Some(emission_type) = emission_type {
            Some(VehicleInfo {emission_type})
        } else {
            None
        };
        Self {
            avoid_tolls, avoid_highways, avoid_ferries, avoid_indoor,vehicle_info,toll_passes
        }
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VehicleInfo {
    emission_type: EmissionType
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmissionType {
    VehicleEmissionTypeUnspecified,
    Gasoline,
    Electric,
    Hybrid,
    Diesel
}

/// A set of values that specify factors to take into consideration when calculating the route.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/RoutingPreference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoutingPreference {
    RoutingPreferenceUnspecified,
    TrafficUnaware,
    TrafficAware,
    TrafficAwareOptimal,
}


/// Values that specify the unit of measure used in the display.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/Units
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Unit {
    UnitsUnspecified,
    Metric,
    Imperial,
}


/// Extra computations to perform while completing the request.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExtraComputation {
    ExtraComputationUnspecified,
    Tolls,
    FuelConsumption,
    TrafficOnPolyline,
    HtmlFormattedNavigationInstructions
}


/// Specifies the assumptions to use when calculating time in traffic.
/// This setting affects the value returned in the duration field in the response, which contains the predicted time in traffic based on historical averages.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TrafficModel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrafficModel {
    TrafficModelUnspecified,
    BestGuess,
    Pessimistic,
    Optimistic,
}

/// Preferences for TRANSIT based routes that influence the route that is returned.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TransitPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitPreferences {
    allowed_travel_modes: Option<Vec<TransitTravelMode>>,
    routing_preference: Option<TransitRoutingPreference>
}

impl TransitPreferences {
    pub fn new(allowed_travel_modes: Option<Vec<TransitTravelMode>>, routing_preference: Option<TransitRoutingPreference>) -> Self {
        Self { allowed_travel_modes, routing_preference }
    }
}



/// A set of values used to specify the mode of transit.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TransitPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransitTravelMode {
    TransitTravelModeUnspecified,
    Bus,
    Subway,
    Train,
    LightRail,
    Rail
}

/// Specifies routing preferences for transit routes.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TransitPreferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransitRoutingPreference {
    TransitRoutingPreferenceUnspecified,
    LessWalking,
    FewerTransfers,
}




/// Information related to how and why a fallback result was used.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/FallbackInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FallbackInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mode: Option<FallbackRoutingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<FallbackReason>
}


/// Actual routing mode used for returned fallback response.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/FallbackInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FallbackRoutingMode {
    FallbackRoutingModeUnspecified,
    FallbackTrafficUnaware,
    FallbackTrafficAware
}

/// Actual routing mode used for returned fallback response.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/FallbackInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FallbackReason {
    FallbackReasonUnspecified,
    ServerError,
    LatencyExceeded
}


/// Contains the additional information that the user should be informed about, such as possible traffic zone restrictions.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/RouteTravelAdvisory
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteTravelAdvisory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toll_info: Option<TollInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_reading_intervals: Option<SpeedReadingInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_consumption_microliters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_restrictions_partially_ignored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_fare: Option<Money>
}


/// Encapsulates toll information on a Route or on a RouteLeg.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TollInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TollInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_price: Option<Vec<Money>>
}

/// Represents an amount of money with its currency type.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/Money
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanos: Option<i128>,
}



/// Traffic density indicator on a contiguous segment of a polyline or path. Given a path with points P_0, P_1, ... , P_N (zero-based index), the SpeedReadingInterval defines an interval and describes its traffic using the following categories.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/SpeedReadingInterval
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedReadingInterval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_polyline_point_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_polyline_point_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>
}

/// The classification of polyline speed based on traffic data.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Speed {
    SpeedUnspecified,
    Normal,
    Slow,
    TrafficJam
}



/// The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>
}

/// Text representations of certain properties.
/// See https://developers.google.com/maps/documentation/routes/reference/rest/v2/TopLevel/computeRoutes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLocalizedValues {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_duration: Option<LocalizedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_fare: Option<LocalizedText>,
}