use std::collections::HashMap;

use config::{self, File, FileFormat, Source};
use eyre::{Ok, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub title: String,
    pub head_link: Option<Vec<HashMap<String, String>>>,
    pub nav: Option<Vec<NavItem>>,
    pub description: Option<String>,
    pub features: Option<Vec<Feature>>,
    pub custom_style_path: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
pub struct NavItem {
    pub title: String,
    pub path: String,
}

#[derive(Deserialize, Clone)]
pub struct Feature {
    pub title: String,
    pub description: String,
}

pub async fn get_configuration() -> Result<Settings> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");

    let mut builder = config::Config::builder().add_source(
        File::from(base_path.join("config"))
            .format(FileFormat::Yaml)
            .required(true),
    );

    for (k, v) in config::Environment::with_prefix("app")
        .separator("__")
        .source(Some(std::env::vars().collect::<HashMap<String, String>>()))
        .collect()?
    {
        builder = builder.set_override(
            k.split('_')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .join("."),
            v,
        )?;
    }

    let mut setting: Settings = builder.build()?.try_deserialize()?;
    setting.application.parse_head_link().await?;
    Ok(setting)
}

impl ApplicationSettings {
    async fn parse_head_link(&mut self) -> Result<()> {
        let head_link = self.head_link.clone().unwrap_or_default();
        let mut head_link_without_scss = Vec::new();
        let mut custom_style_path = Vec::new();
        for link in head_link {
            let mut custom_style = false;
            for (k, v) in &link {
                if k == "href" && v.contains(".scss") {
                    custom_style = true;
                    let scss_path = std::env::current_dir()?.join(v.replace("./", ""));
                    let mut css_path = scss_path.clone();
                    css_path.set_extension("css");
                    let css_path_clone = css_path.clone();
                    grass::from_path(scss_path.clone(), &grass::Options::default())
                        .map(move |css| async { tokio::fs::write(css_path, css).await })?
                        .await?;
                    custom_style_path.push(css_path_clone.to_str().unwrap().to_string())
                }
            }
            if !custom_style {
                head_link_without_scss.push(link);
            }
        }
        self.head_link = Some(head_link_without_scss);
        self.custom_style_path = Some(custom_style_path);
        Ok(())
    }
}
