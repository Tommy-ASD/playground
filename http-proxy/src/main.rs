//! Run with
//!
//! ```not_rust
//! $ cargo run -p example-http-proxy
//! ```
//!
//! In another terminal:
//!
//! ```not_rust
//! $ curl -v -x "127.0.0.1:3000" https://tokio.rs
//! ```
//!
//! Example is based on <https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs>

use crate::bidirectional::copy_bidirectional;
use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::upgrade::Upgraded;
use std::{error::Error, net::SocketAddr};
use tower::{make::Shared, ServiceExt};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use traceback_error::{traceback, TracebackError};

mod bidirectional;
mod upgrade;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_http_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_svc = Router::new().route("/", get(|| async { "Hello, World!" }));

    let service = tower::service_fn(move |req: Request<_>| {
        let router_svc = router_svc.clone();
        let req = req.map(Body::new);
        async move {
            if req.method() == Method::CONNECT {
                proxy(req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    hyper::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service))
        .await
        .unwrap();
}

async fn proxy(req: Request) -> Result<Response, hyper::Error> {
    tracing::trace!(?req);

    if let Some(host_addr) = req.uri().authority().map(|auth| auth.to_string()) {
        tokio::task::spawn(async move {
            match crate::upgrade::upgrade::on(req).await {
                Ok(upgraded) => {
                    if let Err(e) = tunnel(upgraded, host_addr).await {
                        tracing::warn!("server io error: {e}");
                    };
                }
                Err(e) => tracing::warn!("upgrade error: {e}"),
            }
        });

        Ok(Response::new(Body::empty()))
    } else {
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        Ok((
            StatusCode::BAD_REQUEST,
            "CONNECT must be to a socket address",
        )
            .into_response())
    }
}

#[traceback_derive::traceback]
async fn tunnel(
    mut upgraded: crate::upgrade::upgrade::Upgraded,
    addr: String,
) -> Result<(), TracebackError> {
    let mut server = tokio::net::TcpStream::connect(&addr).await?;

    let client_to_server = copy_bidirectional(&mut upgraded, &mut server).await?;

    let (from_client, from_server) = client_to_server;
    tracing::debug!("client wrote {from_client} bytes and received {from_server} bytes");

    Ok(())
}

#[macro_export]
macro_rules! ready {
    ($e:expr $(,)?) => {
        match $e {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
