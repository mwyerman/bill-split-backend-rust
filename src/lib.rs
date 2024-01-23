#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod config;
pub mod data;
pub mod models;


use crate::config::Config;
use crate::api::routes;

pub async fn start_server(config: Config) {
    let app = routes::routes(config);

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
