use anyhow::Result;
use askama::Template;
use clap::{crate_description, crate_version, Arg, ArgMatches, Command};
use reedline_repl_rs::Repl;
use tonic::transport::Channel;

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{
    input_value, AttrInfoRequest, AttrRef, InputInfoRequest, InputSendRequest, InputValue, InputValueType,
    NodeInfoRequest,
};

use crate::output::{AttributeOutput, InputOutput, ListOutput, NodeOutput};
use crate::values::{ColorValue, RangeValue};

mod output;
mod values;

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

    let connect = args.get_one::<String>("connect").expect("Arg with default").clone();

    let context = Context {
        client: InterfaceClient::connect(connect).await?,
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
    client: InterfaceClient<Channel>,
}

async fn nodes(_args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let nodes = context.client.nodes(()).await?;

    return Ok(Some(ListOutput::from(nodes.into_inner()).render().expect("Template error")));
}

async fn inputs(_args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let inputs = context.client.inputs(()).await?;

    return Ok(Some(ListOutput::from(inputs.into_inner()).render().expect("Template error")));
}

async fn root(_args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let node = context.client.root_info(()).await?;

    return Ok(Some(NodeOutput::from(node.into_inner()).render().expect("Template error")));
}

async fn node(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let node = context
        .client
        .node_info(NodeInfoRequest {
            name: args.get_one::<String>("node").expect("Required argument").clone(),
        })
        .await?;

    return Ok(Some(NodeOutput::from(node.into_inner()).render().expect("Template error")));
}

async fn attr(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let attr = context
        .client
        .attr_info(AttrInfoRequest {
            attr: Some(AttrRef {
                node: args.get_one::<String>("node").expect("Required argument").clone(),
                path: args.get_many::<String>("attribute").expect("Required argument").cloned().collect(),
            }),
        })
        .await?;

    return Ok(Some(AttributeOutput::from(attr.into_inner()).render().expect("Template error")));
}

async fn input(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let input = context
        .client
        .input_info(InputInfoRequest {
            name: args.get_one::<String>("input").expect("Required argument").clone(),
        })
        .await?;

    return Ok(Some(InputOutput::from(input.into_inner()).render().expect("Template error")));
}

async fn set(args: ArgMatches, context: &mut Context) -> Result<Option<String>> {
    let input = args.get_one::<String>("input").expect("Required argument").clone();

    // Fetch input info to figure out required value type
    let input = context
        .client
        .input_info(InputInfoRequest {
            name: input,
        })
        .await?
        .into_inner();

    let value = args.get_one::<String>("value").expect("Required argument").clone();

    let value: input_value::Value = match input.value_type() {
        InputValueType::Trigger => input_value::Value::Trigger(()),
        InputValueType::Bool => input_value::Value::Bool(value.parse()?),
        InputValueType::Integer => input_value::Value::Integer(value.parse()?),
        InputValueType::Decimal => input_value::Value::Decimal(value.parse()?),
        InputValueType::Color => input_value::Value::Color(value.parse::<ColorValue>()?.into()),
        InputValueType::IntegerRange => input_value::Value::IntegerRange(value.parse::<RangeValue<i64>>()?.into()),
        InputValueType::DecimalRange => input_value::Value::DecimalRange(value.parse::<RangeValue<f32>>()?.into()),
        InputValueType::ColorRange => input_value::Value::ColorRange(value.parse::<RangeValue<ColorValue>>()?.into()),
    };

    context
        .client
        .input_send(InputSendRequest {
            name: input.name,
            value: Some(InputValue {
                value: Some(value),
            }),
        })
        .await?;

    return Ok(None);
}
