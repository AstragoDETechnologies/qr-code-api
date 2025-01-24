use crate::utils::generate::response::generate_response_from_string;
use axum::{
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::IntoResponse,
};

pub async fn get_generate_epc(
    Query(epc): Query<crate::model::epc::EpcQrCode>,
) -> impl IntoResponse {
    match epc.generate_epc_string() {
        Ok(epc_string) => {
            return generate_response_from_string(&epc_string);
        }
        Err(error) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("An Error occurred: {:?}", error)))
                .unwrap();
        }
    }
}
