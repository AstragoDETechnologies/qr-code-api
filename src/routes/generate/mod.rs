pub mod epc;

use crate::utils::generate::response::generate_response_from_string;
use axum::{
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

    generate_response_from_string(&params.data)
}
