use axum::{http::Uri, routing, Router};

use crate::sk_core::SharpKoalaCore;

use self::paths::*;
use super::response::AxumSkApiResponse;

/// Version handler
pub(crate) mod version;

/// Create the Sharp Koala Axum router
pub fn create_router(core: SharpKoalaCore) -> Router {
    Router::new().route(
        &format!("/{BASE_ROUTE}/{VERSION_ROUTE}"),
        routing::get(version::version_route_handler)
            .with_state(core)
            .fallback(not_found_fallback_handler),
    )
}

/// 404 fallback handler
async fn not_found_fallback_handler(uri: Uri) -> AxumSkApiResponse {
    AxumSkApiResponse::not_found(format!("Route not found: '{uri}'"))
}

pub(super) mod paths {
    pub const BASE_ROUTE: &str = "api";
    pub const VERSION_ROUTE: &str = "version";
}
