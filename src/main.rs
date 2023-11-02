use std::{net::SocketAddr, sync::Arc};

use askama::Template;
use axum::{extract::State, routing::get, Router};
use hyper::server::conn::AddrIncoming;
use rs_htmx::{
    assets,
    items::{self, Item},
    AppState,
};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .nest("/items", items::router())
        .nest("/assets", assets::router())
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Listening on {}", &addr);

    axum::Server::builder(AddrIncoming::bind(&addr).unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    list: Vec<Item>,
}

async fn home(State(state): State<AppState>) -> HomeTemplate {
    let list = state.items.lock().await.to_vec();
    return HomeTemplate { list };
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

async fn about() -> AboutTemplate {
    return AboutTemplate {};
}
