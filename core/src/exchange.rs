use crate::backoff::BackoffPolicy;
use crate::codec::Codec;
use crate::errors::Error;
use crate::normalizer::Normalizer;
use crate::transport::Transport;
use crate::types::{ExchangeId, Subscription};
use async_trait::async_trait;
use url::Url;

#[async_trait]
pub trait ExchangeConnector: Send + Sync {
    fn id(&self) -> ExchangeId;
    fn base_url(&self) -> Url;
    fn subscriptions(&self) -> Vec<Subscription>;
    fn codec(&self) -> Box<dyn Codec>;
    fn normalizer(&self) -> Box<dyn Normalizer>;

    fn backoff_policy(&self) -> BackoffPolicy {
        BackoffPolicy::default()
    }

    async fn bootstrap(&self, transport: &mut dyn Transport) -> Result<(), Error> {
        transport.connect(self.id(), self.base_url()).await?;
        let codec = self.codec();
        let messages = codec.encode_subscribe(&self.subscriptions())?;
        for msg in messages {
            transport.send(msg).await?;
        }
        Ok(())
    }
}
