use axum::{body::StreamBody, extract::Path, response::IntoResponse, routing::get, Router};
use include_dir::{include_dir, Dir};
use tokio_util::io::ReaderStream;

use crate::{
    errors::{Error, ErrorInfo},
    AppState,
};

static ASSETS: Dir<'_> = include_dir!("assets");

pub fn router() -> Router<AppState> {
    return Router::new().route("/*path", get(serve_asset));
}

async fn serve_asset(Path(path): Path<String>) -> Result<impl IntoResponse, Error> {
    let file = ASSETS.get_file(path).ok_or(Error::NotFound(ErrorInfo {
        message: "File not found".to_string(),
        retarget: None,
    }))?;
    let stream = ReaderStream::new(file.contents());
    let body = StreamBody::new(stream);
    return Ok(body);
}
