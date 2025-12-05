use async_trait::async_trait;
use futures::executor::block_on;
use pis_core::errors::Error;
use pis_core::transport::{InboundFrame, OutboundMessage, Transport};
use pis_core::types::ExchangeId;
use pis_sdk::Client;
use url::Url;

struct NoopTransport;

#[async_trait]
impl Transport for NoopTransport {
    async fn connect(&mut self, exchange: ExchangeId, endpoint: Url) -> Result<(), Error> {
        println!("[noop] connect {} -> {}", exchange.0, endpoint);
        Ok(())
    }

    async fn send(&mut self, message: OutboundMessage) -> Result<(), Error> {
        println!("[noop] send {:?}", message);
        Ok(())
    }

    async fn recv(&mut self) -> Result<InboundFrame, Error> {
        Err(Error::Transport("noop transport cannot receive".to_string()))
    }

    async fn disconnect(&mut self) -> Result<(), Error> {
        println!("[noop] disconnect");
        Ok(())
    }
}

fn main() {
    let client = Client::builder().with_binance().build();
    let mut transport = NoopTransport;

    let run = async {
        client.bootstrap_all(&mut transport).await
    };

    if let Err(err) = block_on(run) {
        eprintln!("bootstrap failed: {err}");
    }
}
