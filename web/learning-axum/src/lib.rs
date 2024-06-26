use std::net::IpAddr;

use eyre::Result;
use router::create_main_router;
use tokio::net::TcpListener;

mod router;

#[derive(Debug)]
pub struct App {
    address: IpAddr,
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = IpAddr::from([127, 0, 0, 1]);

        tracing_subscriber::fmt::init();

        Self { address, port }
    }

    pub async fn run(&self) -> Result<()> {
        let router = create_main_router();
        let listener = TcpListener::bind((self.address, self.port)).await?;

        tracing::info!("Server listening @ http://{}:{}", self.address, self.port);

        axum::serve(listener, router).await?;

        Ok(())
    }
}
