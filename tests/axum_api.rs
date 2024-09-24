//! Axum API tests

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::atomic::{AtomicU16, Ordering},
    time::Duration,
};

use sharp_koala::{
    axum::{AxumSharpKoalaApi, AxumSharpKoalaClient},
    SkCoreVersions,
};
use tokio::{
    sync::oneshot::{self, Sender},
    task::JoinHandle,
};

trait UnwrapTest<T> {
    fn unwrap_test(self) -> T;
}

impl<T, E> UnwrapTest<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    #[allow(clippy::unwrap_used)]
    fn unwrap_test(self) -> T {
        self.unwrap()
    }
}

/// Structure to define ports used for the server when testing
static SERVER_TEST_PORT: AtomicU16 = AtomicU16::new(15000);

async fn setup_server() -> (AxumSharpKoalaClient, (Sender<()>, JoinHandle<()>)) {
    let (sender, receiver) = oneshot::channel();
    let port = SERVER_TEST_PORT.fetch_add(1, Ordering::Relaxed);
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let server_task = tokio::spawn(async move {
        let server = AxumSharpKoalaApi::default();
        server.start_server(socket, receiver).await.unwrap_test();
    });
    let _ = tokio::time::sleep(Duration::from_millis(200)).await;
    let client = AxumSharpKoalaClient::new(&format!("http://0.0.0.0:{port}")).unwrap_test();
    (client, (sender, server_task))
}

#[tokio::test]
async fn test_version() {
    let (client, _shutdown) = setup_server().await;
    let version = client.version().await.unwrap_test();
    assert_eq!(version, SkCoreVersions::V0_0_0.version())
}

#[tokio::test]
async fn test_shutdown() {
    let (_, (shutdown, handle)) = setup_server().await;
    shutdown.send(()).unwrap_test();
    handle.await.unwrap_test();
}
