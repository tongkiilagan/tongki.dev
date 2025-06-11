use axum::{Router, routing::get, Extension, response::{Html, IntoResponse}, middleware::{from_fn}, http::{StatusCode}};
use serde::{Serialize};
use serde_json::json;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate<'a> {
    header: &'a str,
    message: &'a str,
}

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
async fn root() -> impl IntoResponse{
    let template = IndexTemplate {
        name: "Test"
    };
    //let html = state.h.render("index", &json!({})).unwrap();
    (StatusCode::OK, Html(template.render().unwrap()).into_response())

}
async fn not_found() -> impl IntoResponse {
    let template = NotFoundTemplate {
        header: "404",
        message: "Page cannot be Found"
    };
    (StatusCode::OK, Html(template.render().unwrap()).into_response())
}