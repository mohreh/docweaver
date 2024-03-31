pub mod configuration;
pub mod router;

use eyre::Result;
use std::net::IpAddr;

use crate::configuration::Settings;
use crate::router::router;

pub struct App {
    addr: IpAddr,
    port: u16,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        let router = router();
        let listener = tokio::net::TcpListener::bind((self.addr, self.port)).await?;

        axum::serve(listener, router).await?;

        Ok(())
    }
}

impl From<Settings> for App {
    fn from(config: Settings) -> Self {
        let addr = "127.0.0.1"
            .parse::<IpAddr>()
            .expect("failed to parse host string to ip address");

        Self {
            addr,
            port: config.application.port,
        }
    }
}
