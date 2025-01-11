pub mod configuration;
pub mod router;
pub mod template;

// use eyre::{Error, Ok, Result};
use std::net::IpAddr;
use tokio::sync::RwLock;

use crate::configuration::{ApplicationSettings, Environment, Settings};
use crate::router::router;

lazy_static::lazy_static! {
    pub static ref SIDEBAR: RwLock<Vec<SidebarItem>> = RwLock::new(Vec::new());
}

pub struct App {
    addr: IpAddr,
    port: u16,
    application_settings: ApplicationSettings,
}

#[derive(Clone)]
pub struct AppState {
    application: ApplicationSettings,
    sidebar: Vec<SidebarItem>,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        let sidebar_items = initialize_sidebar(&self.application_settings).unwrap();
        println!("{:?}", sidebar_items);
        let sidebar = SIDEBAR.read().await;
        for item in sidebar.iter() {
            println!("{:?}", &item);
        }

        let state = AppState {
            application: self.application_settings.clone(),
            sidebar: sidebar_items,
        };

        let router = router(state);
        let listener = tokio::net::TcpListener::bind((self.addr, self.port)).await?;

        axum::serve(listener, router).await?;

        Ok(())
    }
}

impl From<Settings> for App {
    fn from(config: Settings) -> Self {
        let addr = match config.app_environment {
            Environment::Local => "127.0.0.1",
            Environment::Production => "0.0.0.0",
        }
        .parse::<IpAddr>()
        .expect("failed to parse host string to ip address");

        Self {
            addr,
            port: config.port,
            application_settings: config.application,
        }
    }
}

use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Default)]
struct FrontMatter {
    title: Option<String>,
    order: Option<i32>,
    icon: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SidebarItem {
    pub title: String,
    pub path: String,
    pub children: Vec<SidebarItem>,
    pub order: Option<i32>,
    pub icon: Option<String>,
}

pub fn generate_sidebar(docs_path: &Path) -> Result<Vec<SidebarItem>> {
    info!(
        "Starting sidebar generation from path: {}",
        docs_path.display()
    );

    let mut dir_map: std::collections::HashMap<PathBuf, Vec<SidebarItem>> =
        std::collections::HashMap::new();

    // First pass: collect all markdown files
    for entry in WalkDir::new(docs_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().extension().map_or(false, |ext| ext == "md") {
            let path = entry.path();
            let parent = path.parent().unwrap_or(Path::new("")).to_path_buf();

            match process_markdown_file(path, docs_path) {
                Ok(Some(sidebar_item)) => {
                    dir_map.entry(parent).or_default().push(sidebar_item);
                }
                Ok(None) => continue,
                Err(e) => {
                    debug!("Failed to process {}: {}", path.display(), e);
                    continue;
                }
            }
        }
    }

    // Sort items in each directory
    for items in dir_map.values_mut() {
        items.sort_by(|a, b| {
            a.order
                .unwrap_or(i32::MAX)
                .cmp(&b.order.unwrap_or(i32::MAX))
                .then(a.title.cmp(&b.title))
        });
    }

    // Create top-level structure
    let mut result = Vec::new();
    let root_path = docs_path.to_path_buf();

    // Process each directory under root
    for (dir_path, items) in dir_map.iter() {
        if dir_path == &root_path {
            // Add root items directly
            result.extend(items.clone());
        } else if let Ok(relative) = dir_path.strip_prefix(&root_path) {
            if relative.parent().is_none() || relative.parent() == Some(Path::new("")) {
                // This is a top-level directory
                let dir_name = dir_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                // Find index.md in this directory
                let index_item = items.iter().find(|item| item.path.ends_with("index.md"));

                let mut directory_item = if let Some(index) = index_item {
                    // Use index.md's metadata for the directory
                    index.clone()
                } else {
                    // Create default item for directory
                    SidebarItem {
                        title: dir_name,
                        path: relative.to_str().unwrap_or("").to_string(),
                        children: Vec::new(),
                        order: None,
                        icon: None,
                    }
                };

                // Add all non-index items as children
                directory_item.children = items
                    .iter()
                    .filter(|item| !item.path.ends_with("index.md"))
                    .cloned()
                    .collect();

                result.push(directory_item.clone());
            }
        }
    }

    // Sort top-level items
    result.sort_by(|a, b| {
        a.order
            .unwrap_or(i32::MAX)
            .cmp(&b.order.unwrap_or(i32::MAX))
            .then(a.title.cmp(&b.title))
    });

    info!(
        "Completed sidebar generation with {} top-level items",
        result.len()
    );
    Ok(result)
}

fn process_markdown_file(path: &Path, docs_root: &Path) -> Result<Option<SidebarItem>> {
    debug!("Processing markdown file: {}", path.display());

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    debug!("File content length: {} bytes", content.len());
    debug!(
        "First 100 chars: {}",
        &content.chars().take(100).collect::<String>()
    );

    // Split front matter and content
    let (front_matter, _) = parse_front_matter(&content);
    // let front_matter = if content.starts_with("---") {
    //     debug!("Found front matter marker");
    //     if let Some(end_index) = content[3..].find("---") {
    //         let yaml_content = &content[3..end_index + 3];
    //         debug!("Extracted YAML content: {}", yaml_content);
    //         match serde_yaml::from_str::<FrontMatter>(yaml_content) {
    //             Ok(fm) => {
    //                 debug!(
    //                     "Successfully parsed front matter with title: {:?}",
    //                     fm.title
    //                 );
    //                 fm
    //             }
    //             Err(e) => {
    //                 error!("Failed to parse front matter: {}", e);
    //                 FrontMatter::default()
    //             }
    //         }
    //     } else {
    //         warn!("No closing front matter marker found");
    //         FrontMatter::default()
    //     }
    // } else {
    //     debug!("No front matter found");
    //     FrontMatter::default()
    // };

    let relative_path = path
        .strip_prefix(docs_root)
        .with_context(|| format!("Failed to strip prefix from path: {}", path.display()))?;

    let path_str = relative_path
        .to_str()
        .with_context(|| format!("Invalid path: {}", relative_path.display()))?
        .replace('\\', "/");

    // Skip files starting with underscore
    if path
        .file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |n| n.starts_with('_'))
    {
        debug!("Skipping file starting with underscore: {}", path.display());
        return Ok(None);
    }

    let title = front_matter.title.unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .replace('_', " ")
            .to_string()
    });

    debug!(
        "Created sidebar item with title: {} and path: {}",
        title, path_str
    );

    Ok(Some(SidebarItem {
        title,
        path: path_str,
        children: Vec::new(),
        order: front_matter.order,
        icon: front_matter.icon,
    }))
}

fn build_item_hierarchy(
    item: &mut SidebarItem,
    docs_root: &Path,
    dir_map: &mut std::collections::HashMap<PathBuf, Vec<SidebarItem>>,
) {
    debug!("Building hierarchy for item: {}", item.title);

    let item_path = docs_root.join(&item.path);
    let parent_path = item_path.parent().unwrap_or(Path::new("")).to_path_buf();

    if let Some(children) = dir_map.remove(&parent_path) {
        debug!("Found {} children for {}", children.len(), item.title);
        item.children = children;
        for child in &mut item.children {
            build_item_hierarchy(child, docs_root, dir_map);
        }
    } else {
        debug!("No children found for {}", item.title);
    }
}

fn parse_front_matter(content: &str) -> (FrontMatter, String) {
    if content.starts_with("---") {
        if let Some(end_index) = content[3..].find("---") {
            let yaml_content = &content[3..end_index + 3];
            let front_matter: FrontMatter = serde_yaml::from_str(yaml_content).unwrap_or_default();

            // Get the content after front matter, trimming any leading newlines
            let markdown_content = content[end_index + 6..].trim_start().to_string();

            (front_matter, markdown_content)
        } else {
            (FrontMatter::default(), content.to_string())
        }
    } else {
        (FrontMatter::default(), content.to_string())
    }
}

pub fn initialize_sidebar(config: &ApplicationSettings) -> Result<Vec<SidebarItem>> {
    info!("Initializing sidebar");
    let docs_path = Path::new("docs");

    if !docs_path.exists() {
        error!("Documentation directory not found: {}", docs_path.display());
        return Err(anyhow::anyhow!("Documentation directory not found"));
    }

    info!("Found docs directory: {}", docs_path.display());
    generate_sidebar(docs_path)
}
