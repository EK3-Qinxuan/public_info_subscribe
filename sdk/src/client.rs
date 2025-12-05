use pis_core::errors::Error;
use pis_core::exchange::ExchangeConnector;
use pis_core::transport::Transport;

pub struct ClientBuilder {
    connectors: Vec<Box<dyn ExchangeConnector>>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            connectors: Vec::new(),
        }
    }

    pub fn with_connector(mut self, connector: impl ExchangeConnector + 'static) -> Self {
        self.connectors.push(Box::new(connector));
        self
    }

    #[cfg(feature = "binance")]
    pub fn with_binance(self) -> Self {
        self.with_connector(connector_binance::BinanceConnector::new())
    }

    pub fn build(self) -> Client {
        Client {
            connectors: self.connectors,
        }
    }
}

pub struct Client {
    connectors: Vec<Box<dyn ExchangeConnector>>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn connectors(&self) -> &[Box<dyn ExchangeConnector>] {
        &self.connectors
    }

    pub async fn bootstrap_all(&self, transport: &mut dyn Transport) -> Result<(), Error> {
        for connector in &self.connectors {
            connector.bootstrap(transport).await?;
        }
        Ok(())
    }
}
