use anyhow::Result;

pub mod spi;

#[async_trait::async_trait]
pub trait Controller {
    type Config;

    fn new(channels: usize, config: Self::Config) -> Result<Self>
    where
        Self: Sized;

    async fn send(
        &mut self,
        channels: impl Iterator<Item = u8> + Send + 'async_trait,
    ) -> Result<()>;
}
