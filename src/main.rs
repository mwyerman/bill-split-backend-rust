#[macro_use]
extern crate lazy_static;

mod api;
mod config;
mod data;
mod models;


#[tokio::main]
pub async fn main() {
    let config = config::data_config::DataConfig::new();
    let app = api::routes::routes(config);

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

