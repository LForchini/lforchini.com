mod routes;

use axum::handler::Handler;
use axum::{extract::Extension, http, response::IntoResponse, routing::get, Router};
use clap::Parser;
use mongodb::options::ClientOptions;
use mongodb::Client;
use routes::projects;
use shared::Project;
use std::net::SocketAddr;

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
    let projects_collection = database.collection::<Project>("projects");

    if args.initialise {
        log::info!("Initialising DB");
        projects::initialize(projects_collection.clone()).await;
    }

    log::debug!("Creating routes");
    let app = Router::new()
        .fallback(fallback.into_service())
        .route(
            "/projects",
            get(projects::get_projects)
                .post(projects::add_project)
                .put(projects::update_project)
                .delete(projects::remove_project),
        )
        .layer(Extension(projects_collection));

    log::debug!("Running on 127.0.0.1:{:?}", args.port);
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    log::info!("Starting Server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback(uri: http::Uri) -> impl IntoResponse {
    (http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}
