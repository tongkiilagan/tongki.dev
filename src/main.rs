#![allow(non_camel_case_types)]
use axum::{
    Router,
    Extension
};
use tower_http::services::ServeDir;

mod routes;
mod middlewares;
use crate::routes::routes::routes;

#[tokio::main]
async fn main() {

    let app = Router::from(routes())
                .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
