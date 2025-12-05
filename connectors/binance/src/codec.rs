use pis_core::codec::{Codec, DecodedMessage};
use pis_core::errors::Error;
use pis_core::transport::{InboundFrame, OutboundMessage};
use pis_core::types::Subscription;
use serde::Serialize;

#[derive(Default, Clone, Debug)]
pub struct BinanceCodec;

#[derive(Serialize)]
struct SubscribeRequest<'a> {
    method: &'static str,
    params: Vec<String>,
    id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    request_ts: Option<u64>,
}

impl Codec for BinanceCodec {
    fn encode_subscribe(&self, subs: &[Subscription]) -> Result<Vec<OutboundMessage>, Error> {
        let params: Vec<String> = subs.iter().map(to_channel).collect();
        let request = SubscribeRequest {
            method: "SUBSCRIBE",
            params,
            id: 1,
            request_ts: None,
        };
        let payload = serde_json::to_string(&request)
            .map_err(|err| Error::Codec(format!("serialize subscribe: {err}")))?;
        Ok(vec![OutboundMessage::Text(payload)])
    }

    fn decode_inbound(&self, frame: &InboundFrame) -> Result<DecodedMessage, Error> {
        match frame {
            InboundFrame::Text(txt) => {
                let payload = serde_json::from_str(txt)
                    .map_err(|err| Error::Codec(format!("decode text frame: {err}")))?;
                Ok(DecodedMessage { payload })
            }
            InboundFrame::Binary(bin) => {
                let payload: serde_json::Value = serde_json::from_slice(bin)
                    .map_err(|err| Error::Codec(format!("decode binary frame: {err}")))?;
                Ok(DecodedMessage { payload })
            }
        }
    }
}

fn to_channel(sub: &Subscription) -> String {
    let symbol = sub.instrument.symbol.to_lowercase();
    match &sub.kind {
        pis_core::types::SubscriptionKind::Trades => format!("{symbol}@trade"),
        pis_core::types::SubscriptionKind::OrderBookDepth { depth } => {
            format!("{symbol}@depth{depth}")
        }
        pis_core::types::SubscriptionKind::Ticker => format!("{symbol}@ticker"),
        pis_core::types::SubscriptionKind::Candle { interval } => {
            format!("{symbol}@kline_{interval}")
        }
        pis_core::types::SubscriptionKind::Status => format!("{symbol}@miniTicker"),
    }
}
