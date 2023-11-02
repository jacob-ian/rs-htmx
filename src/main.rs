use std::net::SocketAddr;

use axum::{routing::get, Router};
use rs_htmx::assets;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .nest("/assets", assets::router());

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> &'static str {
    return "Hello world";
}
