use std::path::PathBuf;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing, Extension, Router,
};
use comrak::{markdown_to_html, ComrakOptions, ComrakRenderOptions};

#[tokio::main]
async fn main() {
    let app = router().layer(Extension(ComrakOptions {
        ..ComrakOptions::default()
    }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn router() -> Router {
    Router::new().route("/*path", routing::get(handler))
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
