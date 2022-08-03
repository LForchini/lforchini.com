use axum::handler::Handler;
use axum::{extract::Extension, http, response::IntoResponse, routing::get, Router};
use clap::Parser;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::net::SocketAddr;
use std::sync::Arc;

mod project_routes;

#[derive(Debug, Parser)]
struct Args {
    /// Port to run on
    #[clap(short, long, value_parser, default_value_t = 8088)]
    port: u16,

    /// Initialise MongoDB
    #[clap(long = "init")]
    initialise: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Initialising server");

    log::debug!("Reading arguments");
    let args = Args::parse();

    log::debug!("Connecting to MongoDB instance");
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    log::debug!(
        "Found databases: {:?}",
        client.list_database_names(None, None).await.unwrap()
    );

    log::debug!("Connecting to MongoDB Database");
    let database = client.database("lforchini_com");
    let shared_db = Arc::new(database);

    if args.initialise {
        log::info!("Initialising DB");
        let shared_db = Arc::clone(&shared_db);
        project_routes::initialize(shared_db).await;
    }

    log::debug!("Creating routes");
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/", get(root))
        .route(
            "/project",
            get(project_routes::get_all).post(project_routes::post),
        )
        .route(
            "/project/:id",
            get(project_routes::get_one)
                .put(project_routes::put)
                .delete(project_routes::delete),
        )
        .layer(Extension(shared_db));

    log::debug!("Running on 127.0.0.1:{:?}", args.port);
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
