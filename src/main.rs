mod model;
mod routes;
mod utils;

use axum::{routing::get, Router};
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
        .route("/", get(index))
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

async fn index() -> String {
    String::from("The QR-Code API is running!\nNavigate to \"/generate?data=HelloWorld\" to see an example QR-Code.")
}
