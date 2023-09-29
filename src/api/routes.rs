use axum::{
    Router,
    routing::get,
  };
use crate::api::controllers::sum_controller;

pub fn routes() -> Router  {
    Router::new()
        .route("/sum/:number", get(sum_controller::sum))
}