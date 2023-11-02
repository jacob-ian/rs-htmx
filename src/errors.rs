use axum::response::{Html, IntoResponse, Response};
use hyper::StatusCode;

pub enum Error {
    NotFound(String),
    BadRequest(String),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        return match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
        };
    }
    fn get_message(self) -> String {
        return match self {
            Error::NotFound(m) => m,
            Error::BadRequest(m) => m,
        };
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        return (self.get_status_code(), Html(self.get_message())).into_response();
    }
}
