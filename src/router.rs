use comrak::{markdown_to_html, ComrakOptions};
use std::path::PathBuf;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing, Extension, Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/*path", routing::get(handler))
        .layer(Extension(ComrakOptions {
            ..ComrakOptions::default()
        }))
}

async fn handler(
    Path(path): Path<String>,
    Extension(opts): Extension<ComrakOptions>,
) -> impl IntoResponse {
    let dir = PathBuf::from("./docs");
    let mut path = dir.join(path);

    if path.is_dir() {
        path = path.join("index.md");
    } else {
        path.set_extension("md");
    }

    if !path.exists() {
        return Html(StatusCode::NOT_FOUND.to_string());
    }

    let html = markdown_to_html(&tokio::fs::read_to_string(path).await.unwrap(), &opts);

    Html(html)
}
