#![doc = include_str!("../README.md")]

// ---------- Functionality implementation ---------- //

/// Core functionality implementation
mod sk_core;

// --------------- Api implementation --------------- //

/// An [`axum`] implementation of the Sharp Koala API
mod axum_api;

// ----------------- Public objects ----------------- //

/// Axum-related functionality
pub mod axum {
    pub use super::axum_api::{
        client::{AxumSharpKoalaClient, AxumSharpKoalaClientError},
        AxumSharpKoalaApi, AxumSharpKoalaApiError,
    };
}
pub use self::sk_core::{version::SkCoreVersions, SharpKoalaCore};
