use std::sync::Arc;

use askama::Template;
use tokio::sync::Mutex;

pub mod assets;
pub mod errors;

#[derive(Clone, Template)]
#[template(path = "list/item.html")]
pub struct Item {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<Item>>>,
}
