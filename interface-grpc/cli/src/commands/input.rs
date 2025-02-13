use std::fmt;

use anyhow::Result;
use clap::Parser;

use photonic_interface_grpc_client::input::InputSink;
use photonic_interface_grpc_client::values::{ColorValue, RangeValue};
use photonic_interface_grpc_client::InputId;

use crate::commands::CliCommand;
use crate::output::{ListOutput, Output};
use crate::Context;

#[derive(Parser, Debug)]
#[command(name = "input")]
pub enum Input {
    List(InputList),
    Show(InputShow),
    Set(InputSet),
}

impl CliCommand for Input {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        return match args {
            Self::List(args) => InputList::execute(args, context, f).await,
            Self::Show(args) => InputShow::execute(args, context, f).await,
            Self::Set(args) => InputSet::execute(args, context, f).await,
        };
    }
}

#[derive(Parser, Debug)]
#[command(name = "list", about = "List all available inputs")]
pub struct InputList {}

impl CliCommand for InputList {
    async fn execute(_args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        let inputs = context.client.inputs().await?;

        let output = ListOutput::from(inputs.into_iter());
        return Ok(output.render(f)?);
    }
}

#[derive(Parser, Debug)]
#[command(name = "show", about = "Show info about the specified input")]
pub struct InputShow {
    #[arg(required = true)]
    input: InputId,
}

impl CliCommand for InputShow {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        let output = context.client.input(&args.input).await?;

        return Ok(output.render(f)?);
    }
}

#[derive(Parser, Debug)]
#[command(name = "set", about = "Send value to an input")]
pub struct InputSet {
    #[arg(required = true)]
    input: InputId,

    #[arg(required = true)]
    value: String,
}

impl CliCommand for InputSet {
    async fn execute(args: Self, context: &mut Context, f: &mut dyn fmt::Write) -> Result<()> {
        // Fetch input info to figure out required value type
        let input = context.client.input(&args.input).await?;

        match input.sink() {
            InputSink::Trigger(sink) => sink.trigger().await?,
            InputSink::Boolean(sink) => sink.send(args.value.parse()?).await?,
            InputSink::Integer(sink) => sink.send(args.value.parse()?).await?,
            InputSink::Decimal(sink) => sink.send(args.value.parse()?).await?,
            InputSink::Color(sink) => sink.send(args.value.parse::<ColorValue>()?).await?,
            InputSink::IntegerRange(sink) => sink.send(args.value.parse::<RangeValue<i64>>()?).await?,
            InputSink::DecimalRange(sink) => sink.send(args.value.parse::<RangeValue<f32>>()?).await?,
            InputSink::ColorRange(sink) => sink.send(args.value.parse::<RangeValue<ColorValue>>()?).await?,
        };

        return Ok(().render(f)?);
    }
}
