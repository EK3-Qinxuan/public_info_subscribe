use crate::codec::DecodedMessage;
use crate::errors::Error;
use crate::types::NormalizedEvent;

pub trait Normalizer: Send + Sync {
    fn normalize(&self, decoded: DecodedMessage) -> Result<Vec<NormalizedEvent>, Error>;
}
