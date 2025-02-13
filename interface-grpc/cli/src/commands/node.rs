use anyhow::Result;
use clap::Parser;
use std::fmt;

use photonic_interface_grpc_client::NodeId;

use crate::commands::CliCommand;
use crate::output::{ListOutput, Output};
use crate::Context;

#[derive(Parser, Debug)]
#[command(name = "node")]
pub enum Node {
    List(NodeList),
    Root(NodeRoot),
    Show(NodeShow),
}

impl CliCommand for Node {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        return match args {
            Self::List(args) => NodeList::execute(args, context, f).await,
            Self::Root(args) => NodeRoot::execute(args, context, f).await,
            Self::Show(args) => NodeShow::execute(args, context, f).await,
        };
    }
}

#[derive(Parser, Debug)]
#[command(name = "list", about = "List all available nodes")]
pub struct NodeList {}

impl CliCommand for NodeList {
    async fn execute(_args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        let nodes = context.client.nodes().await?;

        let output = ListOutput::from(nodes.into_iter());
        return Ok(output.render(f)?);
    }
}

#[derive(Parser, Debug)]
#[command(name = "root", about = "Show info about root node")]
pub struct NodeRoot {}

impl CliCommand for NodeRoot {
    async fn execute(_args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        let output = context.client.root().await?;

        return Ok(output.render(f)?);
    }
}

#[derive(Parser, Debug)]
#[command(name = "show", about = "Show info about the specified node")]
pub struct NodeShow {
    #[arg(required = true)]
    node: NodeId,
}

impl CliCommand for NodeShow {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        let output = context.client.node(&args.node).await?;

        return Ok(output.render(f)?);
    }
}
