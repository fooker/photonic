use std::sync::Arc;

use tokio::io::{stdin, stdout};

use photonic::interface::{Interface, Introspection};

pub struct CLI;

impl Interface for CLI {
    async fn listen(self, introspection: Arc<Introspection>) -> anyhow::Result<()> {
        let i = stdin();
        let o = stdout();

        return super::run(i, o, introspection).await;
    }
}
