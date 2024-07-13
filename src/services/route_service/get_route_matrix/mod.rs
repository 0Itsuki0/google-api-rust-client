pub mod request_model;
pub mod response_model;

use std::collections::HashMap;
use anyhow::{Ok, Result};
use request_model::{ComputeRouteMatrixRequest, RouteMatrixOrigin};
use reqwest::{header::HeaderValue, Client, Url};
use response_model::ComputeRouteMatrixResponse;
use serde_json::Value;

use super::{RouteService, GET_ROUTE_MATRIX_URL};

impl RouteService {

    /// Calculate the distance and duration of a route for multiple origins and destinations. <br>
    /// See https://developers.google.com/maps/documentation/routes/compute_route_directions
    ///
    /// * `origin` -  Origin waypoint.
    /// * `destination` -  destination waypoint.
    /// * `response_masks` - response field mask. If not specified, all available fields will be included.<br>
    ///     Example: vec!["routes.duration", "routes.distanceMeters"] to return only distanceMeters and duration field for the route.
    ///     See https://developers.google.com/maps/documentation/routes/choose_fields for more details.
    /// * `params` - Optional Additional Parameter. Keys accepted are the following.
    ///    See [ComputeRouteMatixRequestOptinalParams](ComputeRouteMatixRequestOptinalParams) for type detail
    ///     * `intermediates`
    ///     * `travelMode`
    ///     * `routingPreference`
    ///     * `departureTime`: A timestamp in RFC3339 UTC "Zulu" format. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    ///     * `arrivalTime`: A timestamp in RFC3339 UTC "Zulu" format. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    ///     * `languageCode`
    ///     * `regionCode`
    ///     * `units`
    ///     * `extraComputations`
    ///     * `trafficModel`
    ///     * `transitPreferences`
    pub async fn get_route_matrix(&mut self, origin: &Vec<RouteMatrixOrigin>, destination: &Vec<RouteMatrixOrigin>, response_masks: Option<Vec<&str>>, params: Option<HashMap<String, Value>>) -> Result<Vec<ComputeRouteMatrixResponse>>{

        let base_url = Url::parse(&format!("{}", GET_ROUTE_MATRIX_URL))?;
        let mut headers = self.base.create_headers().await?;

        // add field mask
        let mask_string = if let Some(masks) = response_masks {
            masks.join(",")
        } else {
            "*".to_owned()
        };

        headers.insert("X-Goog-FieldMask", HeaderValue::from_str(&mask_string)?);

        let request_body = ComputeRouteMatrixRequest::new(origin, destination, params)?;
        // println!("{}", serde_json::to_string_pretty(&request_body)?);
        let builder: reqwest::RequestBuilder = Client::new().post(base_url)
                .headers(headers)
                .body(serde_json::to_string(&request_body)?);

        let body = self.base.make_request(builder).await?;

        Ok(serde_json::from_str::<Vec<ComputeRouteMatrixResponse>>(&body)?)

    }
}
