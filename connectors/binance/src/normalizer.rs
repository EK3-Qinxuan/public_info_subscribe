use std::collections::BTreeMap;
use std::time::SystemTime;

use pis_core::codec::DecodedMessage;
use pis_core::errors::Error;
use pis_core::normalizer::Normalizer;
use pis_core::types::{NormalizedEvent, StatusUpdate};

#[derive(Default, Clone, Debug)]
pub struct BinanceNormalizer;

impl Normalizer for BinanceNormalizer {
    fn normalize(&self, decoded: DecodedMessage) -> Result<Vec<NormalizedEvent>, Error> {
        // Placeholder implementation: emit a status update carrying the raw payload size.
        let mut details = BTreeMap::new();
        details.insert(
            "raw_bytes".to_string(),
            decoded.payload.to_string().len().to_string(),
        );
        let event = StatusUpdate {
            instrument: None,
            status: "received".to_string(),
            details,
            exchange_ts: SystemTime::now(),
        };
        Ok(vec![NormalizedEvent::Status(event)])
    }
}
