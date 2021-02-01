use std::io::{stdout, Write};
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;

use async_trait::async_trait;
use failure::{Error, Fail};

use photonic_core::attr::AttrValueType;
use photonic_core::color::RGBColor;
use photonic_core::core::*;
use photonic_core::input::InputValueType;
use photonic_core::interface::{AttrInfo, InputInfo, Interface, NodeInfo, Registry, Visitor};
use photonic_grpc_proto as proto;

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
        kind: input.kind.to_string(),
        value_type: into_input_vt(input.value_type).into(),
    };
}

fn into_attr_vt(vt: AttrValueType) -> proto::attr_info::ValueType {
    return match vt {
        AttrValueType::Bool => proto::attr_info::ValueType::Bool,
        AttrValueType::Integer => proto::attr_info::ValueType::Integer,
        AttrValueType::Decimal => proto::attr_info::ValueType::Decimal,
        AttrValueType::Color => proto::attr_info::ValueType::Color,
        AttrValueType::Range(e) => proto::attr_info::ValueType::Range,
    };
}

fn into_input_vt(vt: InputValueType) -> proto::input_info::ValueType {
    return match vt {
        InputValueType::Trigger => proto::input_info::ValueType::Trigger,
        InputValueType::Bool => proto::input_info::ValueType::Bool,
        InputValueType::Integer => proto::input_info::ValueType::Integer,
        InputValueType::Decimal => proto::input_info::ValueType::Decimal,
    };
}

struct InterfaceService {
    registry: Arc<Registry>,
}

#[tonic::async_trait]
impl proto::interface_server::Interface for InterfaceService {
    async fn node_list(&self, request: tonic::Request<proto::NodeListRequest>) -> Result<tonic::Response<proto::NodeListResponse>, tonic::Status> {
        let names = self.registry.nodes.keys().cloned().collect();

        return Ok(tonic::Response::new(proto::NodeListResponse {
            root: self.registry.root.name.clone(),
            names,
        }));
    }

    async fn node_info(&self, request: tonic::Request<proto::NodeInfoRequest>) -> Result<tonic::Response<proto::NodeInfoResponse>, tonic::Status> {
        let node = request.get_ref().name.as_ref().map_or_else(
            || Some(&self.registry.root),
            |name| self.registry.nodes.get(name));

        return Ok(tonic::Response::new(proto::NodeInfoResponse {
            node: node.map(|node| into_node(node.as_ref())),
        }));
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
