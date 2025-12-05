pub mod codec;
pub mod normalizer;

use async_trait::async_trait;
use crate::normalizer::BinanceNormalizer;
use pis_core::backoff::BackoffPolicy;
use pis_core::exchange::ExchangeConnector;
use pis_core::normalizer::Normalizer;
use pis_core::types::{ExchangeId, Instrument, Subscription, SubscriptionKind};
use url::Url;

use crate::codec::BinanceCodec;

pub struct BinanceConnector;

impl BinanceConnector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ExchangeConnector for BinanceConnector {
    fn id(&self) -> ExchangeId {
        ExchangeId("binance".to_string())
    }

    fn base_url(&self) -> Url {
        Url::parse("wss://stream.binance.com:9443/ws").expect("valid ws url")
    }

    fn subscriptions(&self) -> Vec<Subscription> {
        // Minimal defaults for demonstration; users override via SDK builder.
        let instrument = Instrument {
            symbol: "BTCUSDT".to_string(),
            exchange: self.id(),
        };
        vec![
            Subscription {
                instrument: instrument.clone(),
                kind: SubscriptionKind::Trades,
            },
            Subscription {
                instrument,
                kind: SubscriptionKind::OrderBookDepth { depth: 10 },
            },
        ]
    }

    fn codec(&self) -> Box<dyn pis_core::codec::Codec> {
        Box::new(BinanceCodec::default())
    }

    fn normalizer(&self) -> Box<dyn Normalizer> {
        Box::new(BinanceNormalizer::default())
    }

    fn backoff_policy(&self) -> BackoffPolicy {
        BackoffPolicy {
            max_retries: 12,
            base_millis: 500,
            max_millis: 20_000,
        }
    }
}
