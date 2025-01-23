use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use image::Luma;
use qrcode::QrCode;
use std::{error::Error, io::Cursor};

/// Generates a vector of Bytes representing the QR-Code in PNG Format
pub fn generate_png_from_string(data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let code = QrCode::with_error_correction_level(data, qrcode::EcLevel::Q)?;
    let image = code.render::<Luma<u8>>().build();
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();

    Ok(bytes)
}

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
