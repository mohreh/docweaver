use std::{collections::HashMap, path::PathBuf, sync::Once};

use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use eyre::{Ok, Result};

use crate::configuration::{ApplicationSettings, Feature, NavItem, SidebarItem};
static INIT_DOC_STYLES: Once = Once::new();
static INIT_MAINPAGE_STYLE: Once = Once::new();

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
    path: &'a str,
    content: &'a str,
    title: &'a str,
    nav_titles: Vec<NavItem>,
    head_link: Vec<HashMap<String, String>>,
    sidebar: Vec<SidebarItem>,
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
            sidebar: setting.sidebar.clone().unwrap_or_default(),
            path: &path
                .to_str()
                .unwrap_or_default()
                .replace("./docs/", "")
                .replace(".md", ""),
        };

        Ok(html.render().map(|html| {
            INIT_DOC_STYLES.call_once(|| {
                tokio::runtime::Handle::current().spawn(generate_styles(html.clone(), "doc.css"));
            });
            html
        })?)
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

        Ok(html.render().map(|html| {
            INIT_MAINPAGE_STYLE.call_once(|| {
                tokio::runtime::Handle::current().spawn(generate_styles(html.clone(), "index.css"));
            });
            html
        })?)
    }
}

async fn generate_styles(html: String, file_name: &str) -> Result<()> {
    let css = encre_css::generate(html.lines(), &encre_css::config::Config::default());
    let mut style = lightningcss::stylesheet::StyleSheet::parse(
        &css,
        lightningcss::stylesheet::ParserOptions::default(),
    )
    .unwrap();
    style.minify(lightningcss::stylesheet::MinifyOptions::default())?;
    let res = style.to_css(lightningcss::printer::PrinterOptions {
        minify: true,
        ..lightningcss::printer::PrinterOptions::default()
    })?;
    let _ = tokio::fs::write(
        std::env::current_dir()
            .expect("Failed to determine current directory.")
            .join("assets")
            .join("styles")
            .join(file_name),
        res.code,
    )
    .await;
    Ok(())
}
