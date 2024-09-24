use std::str::FromStr;

use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;

use crate::{axum_api::route::version::VersionRouteResponse, sk_core::version::SkCoreVersion};

use super::route::paths::*;

/// All errors returned by the [`AxumSharpKoalaClient`]
#[derive(Debug, thiserror::Error)]
pub enum AxumSharpKoalaClientError {
    /// Invalid server URL
    #[error("Invalid server URL: {0}")]
    InvalidServerUrl(String),
    /// Response de-serialization error
    #[error("Invalid response de-serialization: {0}")]
    ResponseDeserialization(String),
    /// Response parse error
    #[error("Invalid response parsing: {0}")]
    ResponseParse(String),
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(reqwest::Error),
}

/// A client for the [`super::AxumSharpKoalaApi`]
#[derive(Debug)]
pub struct AxumSharpKoalaClient {
    client: Client,
    server_url: Url,
}

impl AxumSharpKoalaClient {
    /// Create a new [`AxumSharpKoalaClient`]
    pub fn new(url: &str) -> Result<Self, AxumSharpKoalaClientError> {
        let server_url = Url::from_str(url)
            .map_err(|err| AxumSharpKoalaClientError::InvalidServerUrl(err.to_string()))?;
        let client = Client::new();
        Ok(Self { client, server_url })
    }

    /// Perform a request to the version method
    pub async fn version(&self) -> Result<SkCoreVersion, AxumSharpKoalaClientError> {
        let path = self.build_path(VERSION_ROUTE);
        let request = self.client.get(path);
        let response = self
            .perform_request::<VersionRouteResponse, SkCoreVersion>(request)
            .await?;
        Ok(response)
    }

    /// Build a path to the server
    fn build_path<T: ToString>(&self, path: T) -> String {
        let inner_path = format!("/{BASE_ROUTE}/{}", path.to_string());
        let mut url = self.server_url.clone();
        url.set_path(&inner_path);
        url.to_string()
    }

    /// Perform an HTTP request to the server, parsing the response
    async fn perform_request<Res, Out>(
        &self,
        req: RequestBuilder,
    ) -> Result<Out, AxumSharpKoalaClientError>
    where
        Res: DeserializeOwned,
        Out: TryFrom<Res, Error = String>,
    {
        let req_debug = format!("{req:?}");
        let raw_response = req
            .send()
            .await
            .map_err(AxumSharpKoalaClientError::Connection)?
            .json::<Res>()
            .await
            .map_err(|_| {
                AxumSharpKoalaClientError::ResponseDeserialization(format!(
                    "Invalid response de-serialization for request '{req_debug}'"
                ))
            })?;
        let response = Out::try_from(raw_response).map_err(|err| {
            AxumSharpKoalaClientError::ResponseParse(format!(
                "Error parsing response for request '{req_debug}': '{err}'"
            ))
        })?;
        Ok(response)
    }
}
