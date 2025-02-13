use std::fmt;

use anyhow::Result;
use clap::Parser;

use photonic_interface_grpc_client::{AttrId, NodeId};

use crate::commands::CliCommand;
use crate::output::Output;
use crate::Context;

#[derive(Parser, Debug)]
#[command(name = "attr")]
pub enum Attr {
    Show(AttrShow),
}

impl CliCommand for Attr {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        return match args {
            Attr::Show(args) => AttrShow::execute(args, context, f).await,
        };
    }
}

#[derive(Parser, Debug)]
#[command(name = "show", about = "Show info about the specified attribute")]
pub struct AttrShow {
    #[arg(required = true)]
    node: NodeId,

    #[arg(required = true, num_args = 1..)]
    path: Vec<String>,
}

impl CliCommand for AttrShow {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        let attribute = AttrId::new(args.node, args.path);
        let output = context.client.attr(&attribute).await?;

        return Ok(output.render(f)?);
    }
}
