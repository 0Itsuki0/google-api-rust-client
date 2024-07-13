
use dotenvy::dotenv;
use google_api_rust_client_unoffical::{auth::service_account::ServiceAccountCredentials, services::route_service::{common_models::{Location, RouteTravelMode, RoutingPreference, TransitPreferences, TransitTravelMode, WayPoint}, get_route_matrix::request_model::RouteMatrixOrigin, RouteService}};
// use google_api_rust_client_unoffical::services::route_service::{get_route::{common_models::{Location, WayPoint, RouteTravelMode}, request_model::{RoutingPreference, TransitPreferences, TransitTravelMode, }, GetRouteResponseFieldMask}, RouteService};
use serde_json::Value;
use std::{collections::HashMap, env, path::PathBuf, str::FromStr};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // get route
    get_route().await?;

    // get route matrix
    get_route_matrix().await?;

    Ok(())
}


async fn get_route() -> Result<()> {
    dotenv().ok();

    // api auth
    let api_key = env::var("API_KEY")?;
    let mut route_service = RouteService::new_with_api_key(api_key);

    let masks = vec!["routes.duration", "routes.distanceMeters"];
    let origin = WayPoint::new_from_location(Location::new(37.419734, -122.0827784, None), None)?;
    let destination = WayPoint::new_from_location(Location::new(35.419734, -100.0827784, None), None)?;

    let mut mid_waypoint_option: HashMap<String, Value> = HashMap::new();
    mid_waypoint_option.insert("via".to_string(), true.into());

    let mut route_option: HashMap<String, Value> = HashMap::new();
    // let mid_waypoint = WayPoint::new_from_place_id("ChIJgUbEo8cfqokR5lP9_Wh_DaM", Some(mid_waypoint_option))?;
    // route_option.insert("intermediates".to_owned(), serde_json::to_value(vec![mid_waypoint.clone()])?);
    route_option.insert("travelMode".to_owned(), serde_json::to_value(RouteTravelMode::Drive)?);
    route_option.insert("routingPreference".to_owned(), serde_json::to_value(RoutingPreference::TrafficAwareOptimal)?);
    // route_option.insert("departureTime".to_owned(), serde_json::to_value(Utc::now() + Duration::seconds(60 * 60 * 24))?);
    // let route_modifiers = RouteModifiers::new(Some(true), Some(true), Some(true), None, None, None);
    // route_option.insert("routeModifiers".to_owned(), serde_json::to_value(route_modifiers)?);
    // route_option.insert("requestedReferenceRoutes".to_owned(), serde_json::to_value(vec![ReferenceRoute::FuelEfficient])?);
    let transit_preference = TransitPreferences::new(Some(vec![TransitTravelMode::Bus]), None);
    route_option.insert("transitPreferences".to_owned(), serde_json::to_value(transit_preference)?);

    let response = route_service.get_route(&origin, &destination, Some(masks.clone()), Some(route_option.clone())).await?;
    println!("response: {}", serde_json::to_string_pretty(&response)?);


    // service account auth
    let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
    let mut route_service = RouteService::new_with_credentials(credentials);
    let response = route_service.get_route(&origin, &destination, Some(masks.clone()), Some(route_option.clone())).await?;
    println!("response: {}", serde_json::to_string(&response)?);

    Ok(())
}


async fn get_route_matrix() -> Result<()> {
    dotenv().ok();

    // api auth
    let api_key = env::var("API_KEY")?;
    let mut route_service = RouteService::new_with_api_key(api_key);

    let masks = vec!["originIndex", "destinationIndex", "status", "condition", "distanceMeters", "duration"];
    let origin = WayPoint::new_from_location(Location::new(37.419734, -122.0827784, None), None)?;
    let destination = WayPoint::new_from_location(Location::new(35.419734, -100.0827784, None), None)?;

    let mut route_option: HashMap<String, Value> = HashMap::new();
    // let mid_waypoint = WayPoint::new_from_place_id("ChIJgUbEo8cfqokR5lP9_Wh_DaM", Some(mid_waypoint_option))?;
    // route_option.insert("intermediates".to_owned(), serde_json::to_value(vec![mid_waypoint.clone()])?);
    route_option.insert("travelMode".to_owned(), serde_json::to_value(RouteTravelMode::Drive)?);

    let response = route_service.get_route_matrix(&vec![RouteMatrixOrigin::new(&origin)], &vec![RouteMatrixOrigin::new(&destination)], Some(masks.clone()), Some(route_option.clone())).await?;
    println!("response: {}", serde_json::to_string_pretty(&response)?);


    // service account auth
    let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
    let mut route_service = RouteService::new_with_credentials(credentials);
    let response = route_service.get_route_matrix(&vec![RouteMatrixOrigin::new(&origin)], &vec![RouteMatrixOrigin::new(&destination)], Some(masks.clone()), Some(route_option.clone())).await?;
    println!("response: {}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
