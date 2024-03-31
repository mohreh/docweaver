use comrak::ComrakOptions;
use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing, Extension, Router,
};

use crate::{
    template::{DocTemplate, MainPageTemplate},
    AppState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(main_page))
        .route("/*path", routing::get(handler))
        .with_state(state)
        .layer(Extension(ComrakOptions {
            ..ComrakOptions::default()
        }))
}

async fn main_page(state: State<AppState>) -> impl IntoResponse {
    Html(
        MainPageTemplate::render_markdown(&state.application)
            .await
            .unwrap(),
    )
}

async fn handler(
    state: State<AppState>,
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

    Html(
        DocTemplate::render_markdown(&state.application, &path, &opts)
            .await
            .unwrap(),
    )
}
