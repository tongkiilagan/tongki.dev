use axum::{
    extract:: Request,
    response::Response,
    middleware::{Next}
};

pub async fn index_middleware(req: Request, next: Next) -> Response {
    println!("Entering middleware");
    let res = next.run(req).await;
    res
}