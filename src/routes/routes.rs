use axum::{Router, routing::get, Extension, response::Html, middleware::{from_fn}};
use crate::{ExtAppState};
use serde::{Serialize};
use serde_json::json;

use crate::middlewares::index_middleware::index_middleware;
#[derive(Serialize)]
struct not_found_msg {
    header: String,
    message: String
}
pub fn routes() -> Router {
    Router::new()
        .route("/", get(root).route_layer(from_fn(index_middleware)))
        .fallback(not_found)
}
async fn root(Extension(state): Extension<ExtAppState>) -> Html<String>{
    let html = state.h.render("index", &json!({})).unwrap();
    Html(html)
}
async fn not_found(Extension(state): Extension<ExtAppState>) -> Html<String> {
    let res = not_found_msg {
        header: String::from("404"),
        message: String::from("Page cannot be Found")
    };
    // optional: serialize into json
    // let json = serde_json::to_string(&res).expect("Failed to not_found mesage convert to JSON");
    let html = state.h.render("404", &res).unwrap();
    Html(html)
}