use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
};
use std::thread;
use uuid::Uuid;
use crate::data::Data;
use crate::config::data_config::DataConfig;

pub async fn get_bills_route(State(config): State<DataConfig>) -> impl IntoResponse {
    thread::spawn(move || {
        let res = config.provider.get_bills();
        Json(res)
    }).join().unwrap()
}

pub async fn get_bill_from_id(
    Path(id): Path<String>,
    State(config): State<DataConfig>
) -> impl IntoResponse {
    thread::spawn(move || {
        let uuid = Uuid::parse_str(&id);
        match uuid {
            Ok(uuid) => {
                let res = config.provider.get_bill(uuid);
                match res {
                    Some(bill) => (StatusCode::OK, Json(bill)).into_response(),
                    None => (StatusCode::NOT_FOUND, Json("Bill not found")).into_response()
                }
            },
            Err(_) => (StatusCode::BAD_REQUEST, Json("Invalid UUID")).into_response()
        }
    }).join().unwrap()
}
