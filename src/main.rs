use std::net::SocketAddr;

use askama::Template;
use axum::{routing::get, Router};
use rs_htmx::assets;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .nest("/assets", assets::router());

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

async fn home() -> HomeTemplate {
    return HomeTemplate {};
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

async fn about() -> AboutTemplate {
    return AboutTemplate {};
}
