use axum::{
    extract,
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
};
use std::thread;
use uuid::Uuid;
use crate::data::Data;
use crate::config::Config;
use crate::models::bill::Bill;

pub async fn get_bills_route(State(config): State<Config>) -> impl IntoResponse {
    thread::spawn(move || {
        let res = config.data.provider.get_bills();
        axum::Json(res)
    }).join().unwrap()
}

pub async fn get_bill_from_id(
    Path(id): Path<String>,
    State(config): State<Config>
) -> impl IntoResponse {
    thread::spawn(move || {
        let uuid = Uuid::parse_str(&id);
        match uuid {
            Ok(uuid) => {
                let res = config.data.provider.get_bill(uuid);
                match res {
                    Some(bill) => (StatusCode::OK, axum::Json(bill)).into_response(),
                    None => (StatusCode::NOT_FOUND, axum::Json("Bill not found")).into_response()
                }
            },
            Err(_) => (StatusCode::BAD_REQUEST, axum::Json("Invalid UUID")).into_response()
        }
    }).join().unwrap()
}

pub async fn new_empty_bill(
    State(config): State<Config>,
    extract::Json(name): extract::Json<String>,
) -> impl IntoResponse {
    thread::spawn(move || {
        let bill = Bill::new(name);
        let res = config.data.provider.add_bill(&bill);
        axum::Json(res)
    }).join().unwrap()
}

pub async fn create_bill(
    State(config): State<Config>,
    extract::Json(bill): extract::Json<crate::models::bill::Bill>,
) -> impl IntoResponse {
    thread::spawn(move || {
        let res = config.data.provider.add_bill(&bill);
        axum::Json(res)
    }).join().unwrap()
}


pub async fn update_bill(
    Path(id): Path<String>,
    State(config): State<Config>,
    extract::Json(bill): extract::Json<crate::models::bill::Bill>,
) -> impl IntoResponse {
    thread::spawn(move || {
        let uuid = Uuid::parse_str(&id);
        match uuid {
            Ok(uuid) => {
                let res = config.data.provider.update_bill(uuid, &bill);
                match res {
                    Ok(uuid) => (StatusCode::OK, axum::Json(uuid)).into_response(),
                    Err(_) => (StatusCode::NOT_FOUND, axum::Json("Bill not found")).into_response()
                }
            },
            Err(_) => (StatusCode::BAD_REQUEST, axum::Json("Invalid UUID")).into_response()
        }
    }).join().unwrap()
}


pub async fn delete_bill(
    Path(id): Path<String>,
    State(config): State<Config>
) -> impl IntoResponse {
    thread::spawn(move || {
        let uuid = Uuid::parse_str(&id);
        match uuid {
            Ok(uuid) => {
                let res = config.data.provider.delete_bill(uuid);
                match res {
                    Ok(uuid) => (StatusCode::OK, axum::Json(uuid)).into_response(),
                    Err(_) => (StatusCode::NOT_FOUND, axum::Json("Bill not found")).into_response()
                }
            },
            Err(_) => (StatusCode::BAD_REQUEST, axum::Json("Invalid UUID")).into_response()
        }
    }).join().unwrap()
}
