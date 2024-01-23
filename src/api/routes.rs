use crate::api::handlers::basic_handler;
use crate::api::handlers::bill_handler;
use crate::config::data_config::DataConfig;
use axum::{
    routing::get,
    Router,
};


pub fn routes(config: DataConfig) -> Router {
    Router::new()
        .fallback(basic_handler::fallback)
        .route("/",
            get(basic_handler::hello)
        ).route("/bills",
            get(bill_handler::get_bills_route)
        ).route("/bill/:id",
            get(bill_handler::get_bill_from_id)
        ).with_state(config)
}
