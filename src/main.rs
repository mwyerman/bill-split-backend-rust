#[macro_use]
extern crate lazy_static;

use axum::{
    routing::{get, post},
    Json,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};
mod bill;
mod data;
use crate::data::Data;
use uuid::Uuid;

use std::thread;


lazy_static! {
    static ref DATA: Data = Data::new();
}


#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/",
            get(hello)
        ).route("/bills",
            get(get_bills_route)
        ).route("/bill/:id",
            get(get_bill_from_id)
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

pub async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("404 Not Found")).into_response()
}

pub async fn hello() -> String {
    "Hello, World!".into()
}

async fn get_bills_route() -> impl IntoResponse {
    thread::spawn(move || {
        let data = DATA.get_bills();
        axum::Json(data)
    }).join().unwrap()
}

async fn get_bill_from_id(
    Path(id): Path<String>
) -> impl IntoResponse {
    thread::spawn(move || {
        let uuid = Uuid::parse_str(&id);
        if uuid.is_err() {
            return (StatusCode::BAD_REQUEST, Json("Invalid UUID")).into_response();
        }
        let data = DATA.get_bill(uuid.unwrap());
        match data {
            Some(bill) => (StatusCode::OK, Json(bill)).into_response(),
            None => (StatusCode::NOT_FOUND, Json("Bill not found")).into_response()
        }
    }).join().unwrap()
}
