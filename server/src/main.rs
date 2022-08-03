use axum::handler::Handler;
use axum::{http, response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;

#[derive(Debug, Parser)]
struct Args {
    /// Port to run on
    #[clap(short, long, value_parser, default_value_t = 8088)]
    port: u16,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Initialising server");

    log::debug!("Reading arguments");
    let args = Args::parse();

    log::debug!("Creating routes");
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", get(root));

    log::debug!("Running on 127.0.0.1:{:#?}", args.port);
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    log::info!("Starting Server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn fallback(uri: http::Uri) -> impl IntoResponse {
    (http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}
