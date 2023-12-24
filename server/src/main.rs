use axum::routing::get;
use axum::{response::IntoResponse, Router};
use tokio::net::TcpListener;

pub fn routes_all() -> Router {
    return Router::new().route("/", get(hello));
}

pub async fn run() {
    // build out application with a single route
    let app = routes_all();
    // run it with hyper on localhost::3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    return run().await;
}

async fn hello() -> impl IntoResponse {
    return "hello from the server";
}
