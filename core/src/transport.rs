use crate::errors::Error;
use crate::types::ExchangeId;
use async_trait::async_trait;
use url::Url;

#[derive(Clone, Debug)]
pub enum OutboundMessage {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum InboundFrame {
    Text(String),
    Binary(Vec<u8>),
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self, exchange: ExchangeId, endpoint: Url) -> Result<(), Error>;
    async fn send(&mut self, message: OutboundMessage) -> Result<(), Error>;
    async fn recv(&mut self) -> Result<InboundFrame, Error>;
    async fn disconnect(&mut self) -> Result<(), Error>;
}
