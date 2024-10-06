#![feature(impl_trait_in_assoc_type)]

use std::pin::Pin;
use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use tonic::codegen::tokio_stream::Stream;
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status};

use photonic::attr::Range;
use photonic::input::{InputSink, InputValueType};
use photonic::interface::{Interface, Introspection};
use photonic_interface_grpc_proto::input_value::{ColorRange, DecimalRange, IntegerRange, Rgb};
use photonic_interface_grpc_proto::interface_server::InterfaceServer;
use photonic_interface_grpc_proto::{
    input_value, interface_server, AttrInfoRequest, AttrInfoResponse, InputInfoRequest, InputInfoResponse,
    InputSendRequest, InputSubscribeRequest, InputValue, InputsResponse, NodeInfoRequest, NodeInfoResponse,
    NodesResponse,
};

pub struct GRPC {}

impl GRPC {
    pub fn new() -> Result<Self> {
        return Ok(Self {});
    }
}

impl Interface for GRPC {
    async fn listen(self, introspection: Arc<Introspection>) -> Result<()> {
        let addr = "[::1]:51889".parse().expect("Invalid interface");

        Server::builder()
            .add_service(InterfaceServer::new(InterfaceImpl {
                introspection,
            }))
            .serve(addr)
            .await?;

        return Ok(());
    }
}

struct InterfaceImpl {
    introspection: Arc<Introspection>,
}

#[async_trait]
impl interface_server::Interface for InterfaceImpl {
    type InputSubscribeStream = Pin<Box<dyn Stream<Item = Result<InputValue, Status>> + Send + 'static>>;

    async fn nodes(&self, _request: Request<()>) -> Result<Response<NodesResponse>, Status> {
        let nodes = self.introspection.nodes.keys().cloned().collect();

        return Ok(Response::new(NodesResponse {
            nodes,
        }));
    }

    async fn inputs(&self, _request: Request<()>) -> Result<Response<InputsResponse>, Status> {
        let inputs = self.introspection.inputs.keys().cloned().collect();

        return Ok(Response::new(InputsResponse {
            inputs,
        }));
    }
    async fn root_info(&self, _request: Request<()>) -> Result<Response<NodeInfoResponse>, Status> {
        let root = &*self.introspection.root;

        return Ok(Response::new(NodeInfoResponse {
            kind: root.kind().to_string(),
            name: root.name().to_string(),
            nodes: root.nodes().iter().map(|(name, info)| (name.clone(), info.name().to_owned())).collect(),
            attrs: root.attrs().keys().cloned().collect(),
        }));
    }

    async fn node_info(&self, request: Request<NodeInfoRequest>) -> Result<Response<NodeInfoResponse>, Status> {
        let request = request.get_ref();

        let node = &**self
            .introspection
            .nodes
            .get(&request.name)
            .ok_or_else(|| Status::not_found(format!("No such node: {}", request.name)))?;

        return Ok(Response::new(NodeInfoResponse {
            kind: node.kind().to_string(),
            name: node.name().to_string(),
            nodes: node.nodes().iter().map(|(name, info)| (name.clone(), info.name().to_owned())).collect(),
            attrs: node.attrs().keys().cloned().collect(),
        }));
    }

    async fn attr_info(&self, request: Request<AttrInfoRequest>) -> Result<Response<AttrInfoResponse>, Status> {
        let request = request.get_ref();

        let attr_ref = request.attr.as_ref().ok_or_else(|| Status::invalid_argument("Value missing: attr"))?;

        let node = &**self
            .introspection
            .nodes
            .get(&attr_ref.node)
            .ok_or_else(|| Status::not_found(format!("No such node: {}", attr_ref.node)))?;

        let attr = node.find_attr(attr_ref.path.iter()).ok_or_else(|| {
            Status::not_found(format!("No such attribute: {}/{}", attr_ref.node, attr_ref.path.join("/")))
        })?;

        return Ok(Response::new(AttrInfoResponse {
            attr: Some(attr_ref.clone()),
            kind: attr.kind().to_string(),
            value_type: attr.value_type().to_string(),
            attrs: attr.attrs().keys().cloned().collect(),
            inputs: attr.inputs().iter().map(|(name, info)| (name.clone(), info.name().to_owned())).collect(),
        }));
    }

    async fn input_info(&self, request: Request<InputInfoRequest>) -> Result<Response<InputInfoResponse>, Status> {
        let request = request.get_ref();

        let input = &**self
            .introspection
            .inputs
            .get(&request.name)
            .ok_or_else(|| Status::not_found(format!("No such input: {}", request.name)))?;

        return Ok(Response::new(InputInfoResponse {
            name: input.name().to_string(),
            value_type: match input.value_type() {
                InputValueType::Trigger => photonic_interface_grpc_proto::InputValueType::Trigger,
                InputValueType::Boolean => photonic_interface_grpc_proto::InputValueType::Bool,
                InputValueType::Integer => photonic_interface_grpc_proto::InputValueType::Integer,
                InputValueType::Decimal => photonic_interface_grpc_proto::InputValueType::Decimal,
                InputValueType::Color => photonic_interface_grpc_proto::InputValueType::Color,
                InputValueType::IntegerRange => photonic_interface_grpc_proto::InputValueType::IntegerRange,
                InputValueType::DecimalRange => photonic_interface_grpc_proto::InputValueType::DecimalRange,
                InputValueType::ColorRange => photonic_interface_grpc_proto::InputValueType::ColorRange,
            }
            .into(),
        }));
    }

    async fn input_send(&self, request: Request<InputSendRequest>) -> Result<Response<()>, Status> {
        let request = request.get_ref();

        let input = &**self
            .introspection
            .inputs
            .get(&request.name)
            .ok_or_else(|| Status::not_found(format!("No such input: {}", request.name)))?;

        macro_rules! match_value {
            ($pattern:ident) => {
                match request
                    .value
                    .as_ref()
                    .ok_or(Status::invalid_argument("Value missing"))?
                    .value
                    .as_ref()
                    .ok_or(Status::invalid_argument("Value missing"))?
                {
                    input_value::Value::$pattern(ref value) => value,
                    _ => return Err(Status::invalid_argument("Value of wrong type")),
                }
            };
        }

        match &input.sink() {
            InputSink::Trigger(sink) => {
                match_value!(Trigger);
                sink.send(())
            }

            InputSink::Boolean(sink) => {
                let value = match_value!(Bool);
                sink.send(*value)
            }

            InputSink::Integer(sink) => {
                let value = match_value!(Integer);
                sink.send(*value)
            }

            InputSink::Decimal(sink) => {
                let value = match_value!(Decimal);
                sink.send(*value)
            }

            InputSink::Color(sink) => {
                let value = match_value!(Color);
                let value = palette::Srgb::new(value.r, value.g, value.b);
                sink.send(value)
            }

            InputSink::IntegerRange(sink) => {
                let value = match_value!(IntegerRange);
                let value = Range::new(value.a, value.b);
                sink.send(value)
            }

            InputSink::DecimalRange(sink) => {
                let value = match_value!(DecimalRange);
                let value = Range::new(value.a, value.b);
                sink.send(value)
            }

            InputSink::ColorRange(sink) => {
                let value = match_value!(ColorRange);
                let value_a = value.a.as_ref().ok_or(Status::invalid_argument("Value missing"))?;
                let value_b = value.b.as_ref().ok_or(Status::invalid_argument("Value missing"))?;
                let value = Range::new(
                    palette::Srgb::new(value_a.r, value_a.g, value_a.b),
                    palette::Srgb::new(value_b.r, value_b.g, value_b.b),
                );
                sink.send(value)
            }
        }
        .map_err(|err| Status::invalid_argument(format!("Invalid value: {}", err)))?;

        return Ok(Response::new(()));
    }

    async fn input_subscribe(
        &self,
        request: Request<InputSubscribeRequest>,
    ) -> Result<Response<Self::InputSubscribeStream>, Status> {
        let request = request.get_ref();

        let input = &**self
            .introspection
            .inputs
            .get(&request.name)
            .ok_or_else(|| Status::not_found(format!("No such input: {}", request.name)))?;

        let stream: Pin<Box<dyn Stream<Item = Result<_, _>> + Send + 'static>> = match &input.sink() {
            InputSink::Trigger(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::Trigger(value)),
                })
            })),

            InputSink::Boolean(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::Bool(value)),
                })
            })),

            InputSink::Integer(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::Integer(value)),
                })
            })),

            InputSink::Decimal(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::Decimal(value)),
                })
            })),

            InputSink::Color(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::Color(Rgb {
                        r: value.red,
                        g: value.green,
                        b: value.blue,
                    })),
                })
            })),

            InputSink::IntegerRange(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::IntegerRange(IntegerRange {
                        a: value.0,
                        b: value.1,
                    })),
                })
            })),

            InputSink::DecimalRange(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::DecimalRange(DecimalRange {
                        a: value.0,
                        b: value.1,
                    })),
                })
            })),

            InputSink::ColorRange(sink) => Box::pin(sink.subscribe().map(|value| {
                Ok(InputValue {
                    value: Some(input_value::Value::ColorRange(ColorRange {
                        a: Some(Rgb {
                            r: value.0.red,
                            g: value.0.green,
                            b: value.0.blue,
                        }),
                        b: Some(Rgb {
                            r: value.1.red,
                            g: value.1.green,
                            b: value.1.blue,
                        }),
                    })),
                })
            })),
        };

        return Ok(Response::new(stream));
    }
}
