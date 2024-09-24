use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

use crate::{
    axum_api::response::AxumSkApiResponse,
    sk_core::{version::SkCoreVersion, SharpKoalaCore},
};

/// Version handler
pub async fn version_route_handler(State(core): State<SharpKoalaCore>) -> AxumSkApiResponse {
    let version = core.version();
    let response = VersionRouteResponse::new(
        version.serial_number().to_string(),
        version.functionalities().to_vec(),
    );
    AxumSkApiResponse::from(response)
}

/// The response for the version route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRouteResponse {
    /// Serial number
    serial_number: String,
    /// Functionality array
    functionalities: Vec<String>,
}

impl VersionRouteResponse {
    /// Create a new [`VersionRouteResponse`]
    pub fn new(serial_number: String, functionalities: Vec<String>) -> Self {
        Self {
            serial_number,
            functionalities,
        }
    }
}

impl From<VersionRouteResponse> for AxumSkApiResponse {
    fn from(value: VersionRouteResponse) -> Self {
        Self::new(StatusCode::OK, json!(value))
    }
}

impl TryFrom<VersionRouteResponse> for SkCoreVersion {
    type Error = String;

    fn try_from(value: VersionRouteResponse) -> Result<Self, Self::Error> {
        let serial = u64::from_str(&value.serial_number).map_err(|err| err.to_string())?;
        let version = SkCoreVersion::new(serial, value.functionalities);
        Ok(version)
    }
}
