use std::path::PathBuf;

use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use eyre::Result;

use crate::configuration::ApplicationSettings;

#[derive(Template)]
#[template(path = "index.html")]
pub struct DocTemplate<'a> {
    content: &'a str,
    title: &'a str,
}

impl<'a> DocTemplate<'a> {
    pub async fn render_markdown(
        setting: &ApplicationSettings,
        path: &PathBuf,
        opts: &ComrakOptions,
    ) -> Result<String> {
        let html = DocTemplate {
            content: &markdown_to_html(&tokio::fs::read_to_string(path).await.unwrap(), opts),
            title: &setting.title,
        };

        Ok(html.render()?)
    }
}
