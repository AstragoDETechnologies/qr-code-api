use std::io::Cursor;

use axum::{
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use image::Luma;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub data: String,
}

pub async fn get_generate(Query(params): Query<Params>) -> impl IntoResponse {
    if &params.data == "" {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("The data-query must not be empty."))
            .unwrap();
    }

    let code = match QrCode::with_error_correction_level(&params.data, qrcode::EcLevel::Q) {
        Ok(code) => code,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(e.to_string()))
                .unwrap();
        }
    };
    let image = code.render::<Luma<u8>>().build();
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(Body::from(bytes))
        .unwrap()
}
