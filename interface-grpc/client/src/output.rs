use askama::Template;
use photonic_interface_grpc_proto::{
    AttrInfoResponse, InputInfoResponse, InputValueType, InputsResponse, NodeInfoResponse, NodesResponse,
};
use std::collections::HashMap;
use yansi::{Paint, Style};

const NODE_STYLE: Style = Style::new().bright_yellow();
const ATTR_STYLE: Style = Style::new().bright_cyan();
const INPUT_STYLE: Style = Style::new().bright_magenta();

const TYPE_STYLE: Style = Style::new().bright_blue();

#[derive(Template)]
#[template(source = "{{ self.0.paint(NODE_STYLE) }}", ext = "txt")]
pub struct NodeName(String);

#[derive(Template)]
#[template(source = "{{ self.0.paint(ATTR_STYLE) }}", ext = "txt")]
pub struct AttrName(String);

#[derive(Template)]
#[template(source = "{{ self.0.paint(INPUT_STYLE) }}", ext = "txt")]
pub struct InputName(String);

#[derive(Template)]
#[template(source = "{{ self.0.paint(TYPE_STYLE) }}", ext = "txt")]
pub struct ValueType(String);

impl From<InputValueType> for ValueType {
    fn from(value_type: InputValueType) -> Self {
        return Self(
            match value_type {
                InputValueType::Trigger => "trigger",
                InputValueType::Bool => "bool",
                InputValueType::Integer => "integer",
                InputValueType::Decimal => "decimal",
                InputValueType::Color => "color",
                InputValueType::IntegerRange => "range<integer>",
                InputValueType::DecimalRange => "range<decimal>",
                InputValueType::ColorRange => "range<color>",
            }
            .to_string(),
        );
    }
}

#[derive(Template)]
#[template(path = "list.txt")]
pub struct ListOutput<T: Template> {
    elements: Vec<T>,
}

impl From<NodesResponse> for ListOutput<NodeName> {
    fn from(response: NodesResponse) -> Self {
        return Self {
            elements: response.nodes.into_iter().map(NodeName).collect(),
        };
    }
}

impl From<InputsResponse> for ListOutput<InputName> {
    fn from(response: InputsResponse) -> Self {
        return Self {
            elements: response.inputs.into_iter().map(InputName).collect(),
        };
    }
}

#[derive(Template)]
#[template(path = "node.txt")]
pub struct NodeOutput {
    name: String,
    kind: String,
    nodes: HashMap<String, NodeName>,
    attrs: Vec<AttrName>,
}

impl From<NodeInfoResponse> for NodeOutput {
    fn from(response: NodeInfoResponse) -> Self {
        return Self {
            name: response.name,
            kind: response.kind,
            nodes: response.nodes.into_iter().map(|(key, name)| (key, NodeName(name))).collect(),
            attrs: response.attrs.into_iter().map(AttrName).collect(),
        };
    }
}

#[derive(Template)]
#[template(path = "attribute.txt")]
pub struct AttributeOutput {
    node: NodeName,
    path: Vec<AttrName>,
    kind: String,
    value_type: ValueType,
    attrs: Vec<AttrName>,
}

impl From<AttrInfoResponse> for AttributeOutput {
    fn from(response: AttrInfoResponse) -> Self {
        let attr = response.attr.expect("Missing field");

        return Self {
            node: NodeName(attr.node),
            path: attr.path.into_iter().map(AttrName).collect(),
            kind: response.kind,
            value_type: ValueType(response.value_type),
            attrs: response.attrs.into_iter().map(AttrName).collect(),
        };
    }
}

#[derive(Template)]
#[template(path = "input.txt")]
pub struct InputOutput {
    name: InputName,
    value_type: ValueType,
}

impl From<InputInfoResponse> for InputOutput {
    fn from(response: InputInfoResponse) -> Self {
        let value_type = response.value_type();

        return Self {
            name: InputName(response.name),
            value_type: ValueType::from(value_type),
        };
    }
}
