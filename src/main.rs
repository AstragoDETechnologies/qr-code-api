mod model;
mod routes;
mod utils;

use axum::{Router, response::Html, routing::get};
use routes::generate::{epc::get_generate_epc, get_generate, wifi::get_generate_wifi};
use std::error::Error;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing (Logger)
    tracing_subscriber::fmt::init();

    let app: axum::Router = Router::new()
        .route("/", get(get_index))
        .route("/robots.txt", get(get_robots))
        .route("/generate", get(get_generate))
        .route("/generate/epc", get(get_generate_epc))
        .route("/generate/wifi", get(get_generate_wifi))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    info!("Server is listening on port 3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_index() -> Html<&'static str> {
    include_str!("../assets/index.html").into()
}

async fn get_robots() -> &'static str {
    include_str!("../assets/robots.txt").into()
}
