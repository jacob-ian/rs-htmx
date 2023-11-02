use std::{net::SocketAddr, sync::Arc};

use askama::Template;
use axum::{
    extract::State,
    routing::{get, post},
    Form, Router,
};
use hyper::StatusCode;
use rs_htmx::{assets, errors::Error, AppState, Item};
use serde::Deserialize;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let state = AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/items", post(create_item))
        .route("/about", get(about))
        .nest("/assets", assets::router())
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Listening on {}", &addr);

    axum::Server::bind(&addr)
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

#[derive(Deserialize)]
struct CreateItem {
    name: String,
}

async fn create_item(
    State(state): State<AppState>,
    Form(create): Form<CreateItem>,
) -> Result<(StatusCode, Item), Error> {
    let mut items = state.items.lock().await;
    let mut found = false;
    for i in &items.to_vec() {
        if i.name == create.name {
            found = true;
            break;
        }
    }
    if found {
        return Err(Error::BadRequest("Item already exists".to_string()));
    }
    let item = Item {
        id: format!("{}-id", create.name),
        name: create.name,
    };
    items.push(item.clone());
    return Ok((StatusCode::CREATED, item));
}
