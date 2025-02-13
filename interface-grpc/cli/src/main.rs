use anyhow::Result;
use clap::{crate_description, crate_version, Arg, Command, CommandFactory};
use reedline_repl_rs::Repl;

use photonic_interface_grpc_client::Client;

use crate::commands::CliCommand;

mod commands;
mod output;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Command::new("")
        .bin_name("photonic-cli")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("connect")
                .short('c')
                .long("connect")
                .help("Address and port of photonic instance to connect to")
                .default_value("http://[::1]:51889"),
        )
        .get_matches();

    let connect = args.get_one::<String>("connect").expect("Arg with default");

    let context = Context {
        client: Client::connect(connect.parse()?).await?,
    };

    let mut repl = Repl::new(context)
        .with_name("photonic-cli")
        .with_version(crate_version!())
        .with_description(crate_description!())
        .with_command_async(commands::node::Node::command(), commands::node::Node::callback)
        .with_command_async(commands::attr::Attr::command(), commands::attr::Attr::callback)
        .with_command_async(commands::input::Input::command(), commands::input::Input::callback);

    return Ok(repl.run_async().await?);
}

struct Context {
    client: Client,
}
