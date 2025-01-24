use super::png_from_string::generate_png_from_string;
use axum::{
    body::Body,
    http::{Response, StatusCode},
};

/// Generates a server response containing the QR-Code in PNG Format
pub fn generate_response_from_string(data: &str) -> Response<Body> {
    match generate_png_from_string(data) {
        Ok(bytes) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(Body::from(bytes))
                .unwrap();
        }

        Err(error) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error.to_string()))
                .unwrap();
        }
    }
}
