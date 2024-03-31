use std::{collections::HashMap, path::PathBuf};

use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use eyre::Result;

use crate::configuration::{ApplicationSettings, Feature, NavItem};

#[derive(Template)]
#[template(path = "index.html")]
pub struct MainPageTemplate<'a> {
    title: &'a str,
    nav_titles: Vec<NavItem>,
    description: &'a str,
    features: Vec<Feature>,
    head_link: Vec<HashMap<String, String>>,
}

#[derive(Template)]
#[template(path = "doc.html")]
pub struct DocTemplate<'a> {
    content: &'a str,
    title: &'a str,
    nav_titles: Vec<NavItem>,
    head_link: Vec<HashMap<String, String>>,
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
            nav_titles: setting.nav.clone().unwrap_or_default(),
            head_link: setting.head_link.clone().unwrap_or_default(),
        };

        Ok(html.render()?)
    }
}

impl<'a> MainPageTemplate<'a> {
    pub async fn render_markdown(setting: &ApplicationSettings) -> Result<String> {
        let html = MainPageTemplate {
            title: &setting.title,
            nav_titles: setting.nav.clone().unwrap_or_default(),
            description: setting.description.as_deref().unwrap_or_default(),
            features: setting.features.clone().unwrap_or_default(),
            head_link: setting.head_link.clone().unwrap_or_default(),
        };

        Ok(html.render()?)
    }
}
