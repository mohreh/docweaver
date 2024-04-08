use std::collections::HashMap;

use config::{self, File, FileFormat, Source};
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

pub async fn get_configuration() -> Result<Settings, config::ConfigError> {
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

    builder.build()?.try_deserialize()
}
