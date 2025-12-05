use crate::errors::Error;
use crate::transport::{InboundFrame, OutboundMessage};
use crate::types::Subscription;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecodedMessage {
    pub payload: serde_json::Value,
}

pub trait Codec: Send + Sync {
    fn encode_subscribe(&self, subs: &[Subscription]) -> Result<Vec<OutboundMessage>, Error>;
    fn decode_inbound(&self, frame: &InboundFrame) -> Result<DecodedMessage, Error>;
}
