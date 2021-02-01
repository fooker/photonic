use failure::Error;
use clap::Clap;
use crate::client::Client;
use erased_serde::Serialize;

pub mod client;

#[derive(Clap)]
#[clap(name="photonic", author, about, version)]
struct Opts {

    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    Nodes,
    Node {
        name: Option<String>,
    },
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let opts = Opts::parse();

    let mut client = client::grpc::GrpcClient::connect("http://localhost:5764".to_owned()).await?;

    let output: Box<dyn Serialize> = match opts.command {
        Command::Nodes => Box::new(client.nodes().await?),
        Command::Node { name } => Box::new(client.node(name).await?),
    };

    println!("{}", serde_yaml::to_string(&output)?);

    return Ok(());
}