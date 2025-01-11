use comrak::ComrakOptions;
use std::path::PathBuf;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use axum::{
    extract::{Path, State},
    http::{header::CONTENT_TYPE, Method, StatusCode},
    response::{Html, IntoResponse},
    routing, Extension, Router,
};

use crate::{
    template::{DocTemplate, MainPageTemplate},
    AppState,
};

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(["http://localhost:8080".parse().unwrap()]);

    let serve_dir =
        ServeDir::new("./assets").not_found_service(ServeFile::new("./templates/404.html"));

    Router::new()
        .nest_service("/assets", serve_dir.clone())
        .route("/", routing::get(main_page))
        .route("/*path", routing::get(handler))
        .fallback_service(serve_dir)
        .with_state(state)
        .layer(cors)
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

#[axum::debug_handler]
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
        DocTemplate::render_markdown(&state.application, &state.sidebar, &path, &opts)
            .await
            .unwrap(),
    )
}
