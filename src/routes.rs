use axum::{Router, routing::get, Extension, response::Html};
use crate::{ExtAppState};
use serde::{Serialize};
use serde_json::json;

#[derive(Serialize)]
struct not_found_msg {
    header: String,
    message: String
}
pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .fallback(not_found)
}
async fn root(Extension(state): Extension<ExtAppState>) -> Html<String>{
    let html = state.h.render("index", &json!({})).unwrap();
    Html(html)
}
async fn not_found(Extension(state): Extension<ExtAppState>) -> Html<String> {
    let res = not_found_msg {
        header: String::from("404"),
        message: String::from("Page not Found")
    };
    // optional: serialize into json
    // let json = serde_json::to_string(&res).expect("Failed to not_found mesage convert to JSON");
    let html = state.h.render("404", &res).unwrap();
    Html(html)
}