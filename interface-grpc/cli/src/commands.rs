use std::fmt;
use std::future::Future;
use std::pin::Pin;

use anyhow::Result;
use clap::{ArgMatches, CommandFactory, FromArgMatches};

use crate::Context;

pub mod attr;
pub mod input;
pub mod node;

pub trait CliCommand: CommandFactory + FromArgMatches {
    fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> impl Future<Output = Result<()>>;

    fn callback<'ctx>(
        args: ArgMatches,
        context: &'ctx mut Context,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>>> + 'ctx>> {
        return Box::pin(async move {
            let args = Self::from_arg_matches(&args)?;

            let mut buffer = String::new();
            Self::execute(args, context, &mut buffer).await?;

            return Ok(Some(buffer));
        });
    }
}
