use askama::Template;
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Form, Router,
};
use hyper::{HeaderMap, StatusCode};
use serde::Deserialize;

use crate::{
    errors::{Error, ErrorInfo, HtmxError, Toast},
    AppState,
};

#[derive(Clone, Template)]
#[template(path = "list/item.html")]
pub struct Item {
    pub id: String,
    pub name: String,
}

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", post(create_item))
        .route("/:id", delete(delete_item));
}

#[derive(Deserialize)]
struct CreateItem {
    name: String,
}

async fn create_item(
    State(state): State<AppState>,
    Form(create): Form<CreateItem>,
) -> Result<(StatusCode, Item), HtmxError<'static, FormTemplate<'static>>> {
    let mut items = state.items.lock().await;
    let mut exists = false;

    for i in &items.to_vec() {
        if i.name == create.name {
            exists = true;
            break;
        }
    }
    if exists {
        return Err(HtmxError::new(
            FormTemplate {
                error: Some("Name already exists"),
            },
            "#list-add",
            "outerHTML",
        ));
    }
    let item = Item {
        id: format!("{}-id", create.name),
        name: create.name,
    };
    items.push(item.clone());
    return Ok((StatusCode::CREATED, item));
}

#[derive(Template)]
#[template(path = "list/form.html")]
pub struct FormTemplate<'a> {
    error: Option<&'a str>,
}

impl<'a> Default for FormTemplate<'a> {
    fn default() -> Self {
        return FormTemplate { error: None };
    }
}

async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, HtmxError<'static, Toast<'static>>> {
    let mut items = state.items.lock().await;
    let mut found = false;

    let mut out: Vec<Item> = Vec::new();
    for i in &items.to_vec() {
        if i.id == id {
            found = true;
            continue;
        }
        if i.id != id {
            out.push(i.clone());
        }
    }
    if !found {
        return Err(HtmxError {
            body: Toast {
                message: "Item does not exist",
            },
            retarget: "body",
            reswap: "beforeend",
        });
    }

    *items = out;
    return Ok(StatusCode::OK);
}
