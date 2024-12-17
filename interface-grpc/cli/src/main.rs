use anyhow::Result;
use askama::Template;
use clap::{crate_description, crate_version, Arg, ArgMatches, Command};
use reedline_repl_rs::Repl;

use photonic_interface_grpc_client::input::{InputId, InputSink};
use photonic_interface_grpc_client::node::NodeId;
use photonic_interface_grpc_client::values::{ColorValue, RangeValue};
use photonic_interface_grpc_client::Client;

use crate::output::{AttributeOutput, InputName, InputOutput, ListOutput, NodeName, NodeOutput};

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
        .with_command_async(Command::new("nodes").about("List all available nodes"), |args, context| {
            Box::pin(nodes(args, context))
        })
        .with_command_async(Command::new("inputs").about("List all available inputs"), |args, context| {
            Box::pin(inputs(args, context))
        })
        .with_command_async(Command::new("root").about("Show info about root node"), |args, context| {
            Box::pin(root(args, context))
        })
        .with_command_async(
            Command::new("node").about("Show info about the specified node").arg(Arg::new("node").required(true)),
            |args, context| Box::pin(node(args, context)),
        )
        .with_command_async(
            Command::new("attribute")
                .alias("attr")
                .about("Show info about the specified attribute")
                .arg(Arg::new("node").required(true))
                .arg(Arg::new("attribute").required(true).num_args(1..)),
            |args, context| Box::pin(attr(args, context)),
        )
        .with_command_async(
            Command::new("input").about("Show info about the specified input").arg(Arg::new("input").required(true)),
            |args, context| Box::pin(input(args, context)),
        )
        .with_command_async(
            Command::new("set")
                .about("Send value to an input")
                .arg(Arg::new("input").required(true))
                .arg(Arg::new("value").required(true)),
            |args, context| Box::pin(set(args, context)),
        );

    return Ok(repl.run_async().await?);
}

struct Context {
    client: Client,
}

async fn nodes(_args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let nodes = context.client.nodes().await?;

    let output = ListOutput::<NodeName>::from(nodes.into_iter());
    let output = Template::render(&output).expect("Template error");
    return Ok(Some(output));
}

async fn inputs(_args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let inputs = context.client.inputs().await?;

    let output = ListOutput::<InputName>::from(inputs.into_iter());
    let output = Template::render(&output).expect("Template error");
    return Ok(Some(output));
}

async fn root(_args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let node = context.client.root().await?;

    let output = NodeOutput::from(node);
    let output = Template::render(&output).expect("Template error");
    return Ok(Some(output));
}

async fn node(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let node = args.get_one::<NodeId>("node").expect("Required argument").clone();
    let node = context.client.node(&node).await?;

    let output = NodeOutput::from(node);
    let output = Template::render(&output).expect("Template error");
    return Ok(Some(output));
}

async fn attr(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let node = args.get_one::<NodeId>("node").expect("Required argument").clone();
    let path = args.get_many::<String>("attribute").expect("Required argument").cloned().collect();

    let attribute = context.client.attribute(&node, path).await?;

    let output = AttributeOutput::from(attribute);
    let output = Template::render(&output).expect("Template error");
    return Ok(Some(output));
}

async fn input(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let input = args.get_one::<InputId>("input").expect("Required argument").clone();
    let input = context.client.input(&input).await?;

    let output = InputOutput::from(input);
    let output = Template::render(&output).expect("Template error");
    return Ok(Some(output));
}

async fn set(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let input = args.get_one::<InputId>("input").expect("Required argument").clone();

    // Fetch input info to figure out required value type
    let input = context.client.input(&input).await?;

    let value = args.get_one::<String>("value").expect("Required argument").clone();

    match input.sink() {
        InputSink::Trigger(sink) => sink.trigger().await?,
        InputSink::Boolean(sink) => sink.send(value.parse()?).await?,
        InputSink::Integer(sink) => sink.send(value.parse()?).await?,
        InputSink::Decimal(sink) => sink.send(value.parse()?).await?,
        InputSink::Color(sink) => sink.send(value.parse::<ColorValue>()?.into()).await?,
        InputSink::IntegerRange(sink) => sink.send(value.parse::<RangeValue<i64>>()?.into()).await?,
        InputSink::DecimalRange(sink) => sink.send(value.parse::<RangeValue<f32>>()?.into()).await?,
        InputSink::ColorRange(sink) => sink.send(value.parse::<RangeValue<ColorValue>>()?.into()).await?,
    };

    return Ok(None);
}
