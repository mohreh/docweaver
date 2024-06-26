pub mod configuration;
pub mod router;
pub mod template;

use eyre::{Ok, Result};
use std::net::IpAddr;

use crate::configuration::{ApplicationSettings, Environment, Settings};
use crate::router::router;

pub struct App {
    addr: IpAddr,
    port: u16,
    application_settings: ApplicationSettings,
}

#[derive(Clone)]
pub struct AppState {
    application: ApplicationSettings,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        let state = AppState {
            application: self.application_settings.clone(),
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
