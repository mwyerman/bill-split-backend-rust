use crate::api::handlers::basic_handler;
use crate::api::handlers::bill_handler;
use crate::config::Config;
use axum::{
    routing::{get, post},
    Router,
};


pub fn routes(config: Config) -> Router {
    Router::new()
        .fallback(basic_handler::fallback)
        .route("/",
            get(basic_handler::hello)
        ).route("/bills",
            get(bill_handler::get_bills_route)
        ).route("/bill/:id",
            get(bill_handler::get_bill_from_id)
                .delete(bill_handler::delete_bill)
                .put(bill_handler::update_bill)
        ).route("/bill/insert",
            post(bill_handler::create_bill)
        ).route("/bill/new",
            post(bill_handler::new_empty_bill)
        ).with_state(config)
}
