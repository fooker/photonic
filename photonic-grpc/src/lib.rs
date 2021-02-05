use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use failure::Error;

use photonic_core::attr::AttrValueType;
use photonic_core::input::{InputValueType, InputSender};
use photonic_core::interface::{AttrInfo, InputInfo, Interface, NodeInfo, Registry};
use photonic_grpc_proto as proto;
use photonic_grpc_proto::{InputSendRequest, InputSendResponse};
use tonic::{Response, Status, Request};
use photonic_grpc_proto::input_send_request::Value;

fn into_node(node: &NodeInfo) -> proto::NodeInfo {
    let nodes = node.nodes.iter()
        .map(|(k, v)| (k.clone(), v.name.clone()))
        .collect();

    let attrs = node.attrs.iter()
        .map(|(k, v)| (k.clone(), into_attr(v.as_ref())))
        .collect();

    return proto::NodeInfo {
        name: node.name.clone(),
        kind: node.kind.to_string(),
        nodes,
        attrs,
    };
}

fn into_attr(attr: &AttrInfo) -> proto::AttrInfo {
    let attrs = attr.attrs.iter()
        .map(|(k, v)| (k.clone(), into_attr(v.as_ref())))
        .collect();

    let inputs = attr.inputs.iter()
        .map(|(k, v)| (k.clone(), v.name.clone()))
        .collect();

    return proto::AttrInfo {
        kind: attr.kind.to_string(),
        value_type: into_attr_vt(attr.value_type).into(),
        attrs,
        inputs,
    };
}

fn into_input(input: &InputInfo) -> proto::InputInfo {
    return proto::InputInfo {
        name: input.name.clone(),
        // kind: input.kind.to_string(),
        value_type: into_input_vt(input.value_type).into(),
    };
}

fn into_attr_vt(vt: AttrValueType) -> proto::attr_info::ValueType {
    return match vt {
        AttrValueType::Boolean => proto::attr_info::ValueType::Boolean,
        AttrValueType::Integer => proto::attr_info::ValueType::Integer,
        AttrValueType::Decimal => proto::attr_info::ValueType::Decimal,
        AttrValueType::Color => proto::attr_info::ValueType::Color,
        AttrValueType::Range(e) => proto::attr_info::ValueType::Range,
    };
}

fn into_input_vt(vt: InputValueType) -> proto::input_info::ValueType {
    return match vt {
        InputValueType::Trigger => proto::input_info::ValueType::Trigger,
        InputValueType::Boolean => proto::input_info::ValueType::Boolean,
        InputValueType::Integer => proto::input_info::ValueType::Integer,
        InputValueType::Decimal => proto::input_info::ValueType::Decimal,
    };
}

struct InterfaceService {
    registry: Arc<Registry>,
}

#[tonic::async_trait]
impl proto::interface_server::Interface for InterfaceService {
    async fn node_list(&self, _: tonic::Request<proto::NodeListRequest>) -> Result<tonic::Response<proto::NodeListResponse>, tonic::Status> {
        let names = self.registry.nodes.keys().cloned().collect();

        return Ok(tonic::Response::new(proto::NodeListResponse {
            root: self.registry.root.name.clone(),
            names,
        }));
    }

    async fn node_info(&self, request: tonic::Request<proto::NodeInfoRequest>) -> Result<tonic::Response<proto::NodeInfoResponse>, tonic::Status> {
        let request = request.get_ref();

        let node = request.name.as_ref().map_or_else(
            || Some(&self.registry.root),
            |name| self.registry.nodes.get(name));

        return Ok(tonic::Response::new(proto::NodeInfoResponse {
            node: node.map(|node| into_node(node.as_ref())),
        }));
    }

    async fn input_send(&self, request: Request<InputSendRequest>) -> Result<Response<InputSendResponse>, Status> {
        let request = request.get_ref();

        let input = self.registry.inputs.get(&request.name);
        if let Some(input) = input {
            if let Some(ref value) = request.value {
                match (&input.sender, value) {
                    (InputSender::Trigger(sender), proto::input_send_request::Value::Trigger(value)) => {
                        sender.send(());
                    }
                    (InputSender::Boolean(sender), proto::input_send_request::Value::Boolean(value)) => {
                        sender.send(value.value);
                    }
                    (InputSender::Integer(sender), proto::input_send_request::Value::Integer(value)) => {
                        sender.send(value.value);
                    }
                    (InputSender::Decimal(sender), proto::input_send_request::Value::Decimal(value)) => {
                        sender.send(value.value);
                    }
                    (_, _) => {
                        return Err(Status::invalid_argument("Wrong type"));
                    }
                }
            } else {
                return Err(Status::invalid_argument("Value missing"));
            }
        } else {
            return Err(Status::not_found(format!("No such input: {}", request.name)));
        }

        return Ok(tonic::Response::new(InputSendResponse {}));
    }
}

pub struct GrpcInterface {
    address: SocketAddr,
}

impl GrpcInterface {
    pub fn bind(address: SocketAddr) -> Self {
        return Self {
            address,
        };
    }
}

#[async_trait]
impl Interface for GrpcInterface {
    async fn listen(self, registry: Arc<Registry>) -> Result<(), Error> {
        let service = InterfaceService {
            registry,
        };

        tonic::transport::Server::builder()
            .add_service(proto::interface_server::InterfaceServer::new(service))
            .serve(self.address)
            .await?;

        return Ok(());
    }
}
