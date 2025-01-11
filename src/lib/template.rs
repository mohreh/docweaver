use std::{collections::HashMap, path::PathBuf, sync::Once};

use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use eyre::{Ok, Result};

use crate::{
    configuration::{load_icon, ApplicationSettings, Feature, NavItem, QuickLink},
    parse_front_matter, SidebarItem,
};
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
    quick_links: Vec<QuickLink>,
    direction: String,
}

#[derive(Template)]
#[template(path = "doc.html")]
pub struct DocTemplate<'a> {
    path: &'a str,
    content: &'a str,
    title: &'a str,
    nav_titles: Vec<NavItem>,
    head_link: Vec<HashMap<String, String>>,
    sidebar_html: String,
    direction: String,
}

impl<'a> DocTemplate<'a> {
    pub async fn render_markdown(
        setting: &ApplicationSettings,
        sidebar: &Vec<SidebarItem>,
        path: &PathBuf,
        opts: &ComrakOptions,
    ) -> Result<String> {
        let content = &tokio::fs::read_to_string(path).await.unwrap();
        let (_, markdown) = parse_front_matter(&content);
        let html = DocTemplate {
            content: &markdown_to_html(&markdown, opts),
            title: &setting.title,
            nav_titles: setting.nav.clone().unwrap_or_default(),
            head_link: setting.head_link.clone().unwrap_or_default(),
            sidebar_html: generate_sidebar_html(&sidebar, path.to_str().unwrap_or_default(), 0),
            path: &path
                .to_str()
                .unwrap_or_default()
                .replace("./docs/", "")
                .replace(".md", ""),
            direction: setting.direction.clone().unwrap_or("ltr".to_string()),
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
            direction: setting.direction.clone().unwrap_or("ltr".to_string()),
            quick_links: setting.quick_links.clone().unwrap_or_default(),
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

fn generate_sidebar_html(items: &[SidebarItem], current_path: &str, depth: usize) -> String {
    let mut html = String::new();
    for item in items {
        let mut class = if current_path[7..] == item.path {
            "font-bold"
        } else {
            ""
        }
        .to_string();

        if depth > 0 {
            class += " border-l-1 pl-2 border-slate-200";
        }
        html.push_str(&format!(
            "<li class=\"{}\">
                <div class=\"flex flex-row\">
                    {}<a href=\"/{}\" class=\"pl-2\">{}</a>
                </div>
            ",
            class,
            load_icon(item.icon.clone().unwrap_or_default()).unwrap_or_default(),
            item.path,
            item.title,
        ));
        if !item.children.is_empty() {
            html.push_str("<ul class=\"sidebar-item font-normal\">");
            html.push_str(&generate_sidebar_html(
                &item.children,
                current_path,
                depth + 1,
            ));
            html.push_str("</ul>");
        }
        html.push_str("</li>");
    }
    html
}
