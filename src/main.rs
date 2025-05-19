use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use app_state::AppState;
use axum::{Router, routing::get};

use hyper::server::conn::http1;
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{Level, info};

pub mod app_state;
pub mod controllers;
pub mod interfaces;
pub mod models;
pub mod repositories;
pub mod util;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0))
        .await
        .unwrap();

    info!("Listening on http://{}", listener.local_addr().unwrap());

    let router = Router::new()
        .nest(
            "/api",
            Router::new().route(
                "/get_folder_content",
                get(controllers::shared::get_folder_content),
            ),
        )
        .fallback_service(ServeDir::new(util::WEBPAGE_FOLDER_PATH))
        .with_state(AppState::default().await);

    loop {
        let Ok((conn, _)) = listener.accept().await else {
            continue;
        };

        tokio::spawn(
            http1::Builder::new()
                .serve_connection(TokioIo::new(conn), TowerToHyperService::new(router.clone())),
        );
    }
}
