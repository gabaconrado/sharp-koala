use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

/// A response of the Axum API
#[derive(Debug)]
pub struct AxumSkApiResponse {
    code: StatusCode,
    response: Json<Value>,
}

impl AxumSkApiResponse {
    /// Create a new [`AxumSkApiResponse`]
    pub fn new(code: StatusCode, response: Value) -> Self {
        Self {
            code,
            response: Json(response),
        }
    }

    /// Create a not found response
    pub fn not_found<T: ToString>(msg: T) -> Self {
        Self::new(StatusCode::NOT_FOUND, json!({"msg": msg.to_string()}))
    }
}

impl IntoResponse for AxumSkApiResponse {
    fn into_response(self) -> Response {
        (self.code, self.response).into_response()
    }
}
