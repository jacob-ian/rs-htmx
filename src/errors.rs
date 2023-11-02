use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub enum Error {
    NotFound(String),
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        return match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
        };
    }
    fn get_message(self) -> String {
        return match self {
            Error::NotFound(m) => m,
        };
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        return (self.get_status_code(), self.get_message()).into_response();
    }
}
