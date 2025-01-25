use crate::utils::generate::response::generate_response_from_string;
use axum::{
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::IntoResponse,
};

pub async fn get_generate_wifi(
    Query(wifi): Query<crate::model::wifi::WifiQrCode>,
) -> impl IntoResponse {
    match wifi.generate_wifi_string() {
        Ok(wifi_string) => {
            return generate_response_from_string(&wifi_string);
        }
        Err(error) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("An Error occurred: {:?}", error)))
                .unwrap();
        }
    }
}
