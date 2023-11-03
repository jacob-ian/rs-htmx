use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use hyper::{HeaderMap, StatusCode};

#[derive(Template)]
#[template(path = "toast.html")]
pub struct Toast<'a> {
    pub message: &'a str,
}

#[derive(Clone)]
pub struct HtmxError<'a, B>
where
    B: IntoResponse,
{
    pub body: B,
    pub retarget: &'a str,
    pub reswap: &'a str,
}

impl<'a, B> HtmxError<'a, B>
where
    B: IntoResponse,
{
    pub fn new(template: B, retarget: &'a str, reswap: &'a str) -> Self {
        return HtmxError {
            retarget,
            reswap,
            body: template,
        };
    }
    pub fn toast(message: &'a str) -> HtmxError<'_, Toast<'_>> {
        let toast: Toast<'a> = Toast { message };
        return HtmxError {
            body: toast,
            retarget: "body",
            reswap: "beforeend",
        };
    }
}

impl<'a, B> IntoResponse for HtmxError<'a, B>
where
    B: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Retarget", self.retarget.parse().unwrap());
        headers.insert("HX-Reswap", self.reswap.parse().unwrap());
        return (StatusCode::UNPROCESSABLE_ENTITY, headers, self.body).into_response();
    }
}

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
