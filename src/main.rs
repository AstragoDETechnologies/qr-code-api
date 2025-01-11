mod routes;

use axum::{routing::get, Router};
use routes::generate::get_generate;
use std::error::Error;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing (Logger)
    tracing_subscriber::fmt::init();

    let app: axum::Router = Router::new()
        .route("/", get(|| async { "The QR-Code API is running!\nNavigate to \"/generate?data=Hello\" to see an example qr-code." }))
        .route("/generate", get(get_generate));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
