use std::net::SocketAddr;

use tokio::{net::TcpListener, sync::oneshot::Receiver};

use crate::sk_core::SharpKoalaCore;

/// Client implementation
pub(crate) mod client;
/// Response implementation
mod response;
/// Routes implementation
mod route;

/// All errors from the Axum API
#[derive(Debug, thiserror::Error)]
pub enum AxumSharpKoalaApiError {
    /// Server start error
    #[error("Error starting the server: {0}")]
    ServerStart(std::io::Error),
    /// Error during server operation
    #[error("Error during server operation: {0}")]
    ServerOperation(std::io::Error),
}

/// Axum implementation of the Sharp Koala API
#[derive(Debug, Default)]
pub struct AxumSharpKoalaApi {
    /// Core object
    core: SharpKoalaCore,
}

impl AxumSharpKoalaApi {
    /// Create a new [`AxumSharpKoalaApi`]
    pub fn new(core: SharpKoalaCore) -> Self {
        Self { core }
    }

    /// Start the [`AxumSharpKoalaApi`]
    pub async fn start_server(
        self,
        socket: SocketAddr,
        shutdown_receiver: Receiver<()>,
    ) -> Result<(), AxumSharpKoalaApiError> {
        let router = route::create_router(self.core);
        let listener = TcpListener::bind(socket)
            .await
            .map_err(AxumSharpKoalaApiError::ServerStart)?;
        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(async move {
                match shutdown_receiver.await {
                    Ok(..) => tracing::info!("Axum Sharp Koala API gracefully shutdown since it received the signal"),
                    Err(..) => tracing::info!(
                        "Axum Sharp Koala API gracefully shutdown since the signal sender has dropped"
                    ),
                }
            })
            .await
            .map_err(AxumSharpKoalaApiError::ServerOperation)?;
        Ok(())
    }
}
