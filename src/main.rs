mod model;
mod routes;
mod utils;

use axum::{response::Html, routing::get, Router};
use routes::generate::{epc::get_generate_epc, get_generate};
use std::error::Error;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing (Logger)
    tracing_subscriber::fmt::init();

    let cors_layer: CorsLayer = CorsLayer::permissive();

    let app: axum::Router = Router::new()
        .route("/", get(get_index))
        .route("/robots.txt", get(get_robots))
        .route("/generate", get(get_generate))
        .route("/generate/epc", get(get_generate_epc))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_index() -> Html<&'static str> {
    include_str!("../assets/index.html").into()
}

async fn get_robots() -> &'static str {
    include_str!("../assets/robots.txt").into()
}
