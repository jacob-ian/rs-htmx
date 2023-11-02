use std::sync::Arc;

use tokio::sync::Mutex;

use crate::items::Item;

pub mod assets;
pub mod errors;
pub mod items;

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<Item>>>,
}
