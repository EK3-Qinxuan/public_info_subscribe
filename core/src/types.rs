use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExchangeId(pub String);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Instrument {
    pub symbol: String,
    pub exchange: ExchangeId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscriptionKind {
    Trades,
    OrderBookDepth { depth: u32 },
    Ticker,
    Candle { interval: String },
    Status,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub instrument: Instrument,
    pub kind: SubscriptionKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub instrument: Instrument,
    pub price: f64,
    pub size: f64,
    pub side: Side,
    pub exchange_ts: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookDelta {
    pub instrument: Instrument,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
    pub exchange_ts: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub instrument: Instrument,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub exchange_ts: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Candle {
    pub instrument: Instrument,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub interval: String,
    pub exchange_ts: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub instrument: Option<Instrument>,
    pub status: String,
    pub details: BTreeMap<String, String>,
    pub exchange_ts: SystemTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Level {
    pub price: f64,
    pub size: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NormalizedEvent {
    Trade(Trade),
    BookDelta(BookDelta),
    Ticker(Ticker),
    Candle(Candle),
    Status(StatusUpdate),
}
