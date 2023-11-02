use askama::Template;
use axum::{extract::State, routing::post, Form, Router};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    errors::{Error, ErrorInfo},
    AppState,
};

#[derive(Clone, Template)]
#[template(path = "list/item.html")]
pub struct Item {
    pub id: String,
    pub name: String,
}

pub fn router() -> Router<AppState> {
    return Router::new().route("/", post(create_item));
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
        return Err(Error::ValidationError(ErrorInfo {
            message: String::from("Name already exists"),
            retarget: Some(String::from("#list-add")),
        }));
    }
    let item = Item {
        id: format!("{}-id", create.name),
        name: create.name,
    };
    items.push(item.clone());
    return Ok((StatusCode::CREATED, item));
}
