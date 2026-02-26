use dioxus::fullstack::HeaderValue;
use dioxus::fullstack::response::{IntoResponse, Response};
use dioxus::server::http::header;
use dioxus::prelude::*;

pub struct FileResponse;

impl FileResponse {
    pub async fn new(path: &str) -> Result<Response> {
        use tokio::fs;
        
        if fs::metadata(&path).await.is_ok() {
            let filename = path.split("/").last().unwrap_or_default();
            
            let mut body = fs::read(&path).await?.into_response();
            body.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("image/jpg; charset=utf-8"));
            body.headers_mut().insert(header::CONTENT_DISPOSITION, HeaderValue::from_str(&*format!("attachment; filename=\"{filename}\""))?);
            Ok(body)
        } else {
            Ok(StatusCode::NOT_FOUND.into_response())
        }
    }
}
