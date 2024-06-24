use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthErrorResponse {
    pub error: String, 
    pub error_description: String
}

impl Default for AuthErrorResponse {
    fn default() -> Self {
        Self { 
            error: "unknown_error".to_owned(), 
            error_description: "Unknown error encountered.".to_owned()
        }
    }
}