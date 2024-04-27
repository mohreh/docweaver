use std::collections::HashMap;

use config::{self, File, FileFormat, Source};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
    pub app_environment: Environment,
    pub application: ApplicationSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub title: String,
    pub head_link: Option<Vec<HashMap<String, String>>>,
    pub nav: Option<Vec<NavItem>>,
    pub sidebar: Option<Vec<SidebarItem>>,
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
pub struct SidebarItem {
    pub title: String,
    pub path: String,
}

#[derive(Deserialize, Clone)]
pub struct Feature {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment, Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub async fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory.");

    let mut builder = config::Config::builder()
        .add_source(
            File::from(base_path.join("config"))
                .format(FileFormat::Yaml)
                .required(true),
        )
        .add_source(config::Environment::with_prefix("app").separator("__"));

    // for (k, v) in config::Environment::with_prefix("app")
    //     .separator("__")
    //     .source(Some(std::env::vars().collect::<HashMap<String, String>>()))
    //     .collect()?
    // {
    //     builder = builder.set_override(
    //         k.split('_')
    //             .map(|s| s.to_owned())
    //             .collect::<Vec<String>>()
    //             .join("."),
    //         v,
    //     )?;
    // }

    builder.build()?.try_deserialize()
}
