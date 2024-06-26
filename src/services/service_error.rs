use std::vec;

use serde::{Serialize, Deserialize};
use serde_json::Value;


// https://cloud.google.com/apis/design/errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceErrorResponse {
    pub error: ErrorResponseStatus
}

impl Default for ServiceErrorResponse {
    fn default() -> Self {
        Self {
            error: ErrorResponseStatus::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseStatus {
    pub code: u32,
    pub message: String,
    pub details: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Value>>
}

impl Default for ErrorResponseStatus {
    fn default() -> Self {
        Self {
            code: 400,
            message: "Unknown Error".to_owned(),
            details: vec![],
            status: None,
            errors: None
        }
    }
}
