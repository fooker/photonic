#![allow(clippy::needless_return)]

use anyhow::Error;
use clap::Clap;
use erased_serde::Serialize;

use crate::client::Client;

pub mod client;

#[derive(Clap)]
#[clap(name = "photonic", author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    Nodes,
    Node {
        name: Option<String>,
    },
    Send {
        name: String,

        #[clap(subcommand)]
        value: SendValue,
    },
}

#[derive(Clap)]
pub enum SendValue {
    Trigger,
    Boolean {
        #[clap(parse(try_from_str))]
        value: bool,
    },
    Integer {
        #[clap(parse(try_from_str))]
        value: i64,
    },
    Decimal {
        #[clap(parse(try_from_str))]
        value: f64,
    },
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let opts = Opts::parse();

    let mut client = client::grpc::GrpcClient::connect("http://localhost:5764".to_owned()).await?;

    let output: Box<dyn Serialize> = match opts.command {
        Command::Nodes => Box::new(client.nodes().await?),
        Command::Node {
            name,
        } => Box::new(client.node(name).await?),
        Command::Send {
            name,
            value,
        } => Box::new(client.send(name, value).await?),
    };

    println!("{}", serde_yaml::to_string(&output)?);

    return Ok(());
}
