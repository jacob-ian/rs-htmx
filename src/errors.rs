use axum::response::{Html, IntoResponse, Response};
use hyper::{HeaderMap, StatusCode};

pub enum Error {
    NotFound(ErrorInfo),
    BadRequest(ErrorInfo),
    ValidationError(ErrorInfo),
}

pub struct ErrorInfo {
    pub message: String,
    /// the ID of the retarget element
    pub retarget: Option<String>,
}

impl Error {
    fn get_status_code(&self) -> StatusCode {
        return match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
    }
    fn get_message(self) -> String {
        return match self {
            Error::NotFound(e) => e.message,
            Error::BadRequest(e) => e.message,
            Error::ValidationError(e) => e.message,
        };
    }
    fn get_retarget(&self) -> Option<String> {
        let inner = match self {
            Error::BadRequest(i) => i,
            Error::NotFound(i) => i,
            Error::ValidationError(i) => i,
        };
        return inner.retarget.to_owned();
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        // headers.insert("HX-Reswap", "innerHTML".parse().unwrap());
        if let Some(retarget) = self.get_retarget() {
            headers.insert("HX-Retarget", retarget.parse().unwrap());
        };
        return (
            self.get_status_code(),
            headers,
            Html(format!("<span>{}</span>", self.get_message())),
        )
            .into_response();
    }
}
