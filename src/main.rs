#![allow(non_camel_case_types)]
use std::sync::Arc;
use axum::{
    Router,
    Extension
};
use tower_http::services::ServeDir;

mod routes;
mod middlewares;
use crate::routes::routes::routes;

pub struct AppState {
    h: handlebars::Handlebars<'static>,
}
pub type ExtAppState = Arc<AppState>;
#[tokio::main]
async fn main() {
    let mut h = handlebars::Handlebars::new();
    h.register_template_file("index", "templates/index.hbs")
        .expect("Unable to register index template");
    h.register_template_file("404", "templates/404.hbs")
        .expect("Unable to register 404 template");
    let state = Arc::new(AppState { h });

    let app = Router::from(routes())
                .layer(Extension(state))
                .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
