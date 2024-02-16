use photonic::interface::{Interface, Introspection};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct CLI {
    pub address: SocketAddr,
}

impl Interface for CLI {
    async fn listen(self, introspection: Arc<Introspection>) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;

        loop {
            let introspection = introspection.clone();

            let (mut stream, _remote) = listener.accept().await?;

            tokio::spawn(async move {
                let (i, o) = stream.split();
                super::run(i, o, introspection).await
            });
        }
    }
}
