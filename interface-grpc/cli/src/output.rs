use std::collections::HashMap;

use askama::Template;
use yansi::{Paint, Style};

use photonic_interface_grpc_client::attr::Attribute;
use photonic_interface_grpc_client::input::Input;
use photonic_interface_grpc_client::node::{Node, NodeId};
use photonic_interface_grpc_client::{values, Identified, Ref};

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

impl From<values::ValueType> for ValueType {
    fn from(value_type: values::ValueType) -> Self {
        return Self(
            match value_type {
                values::ValueType::Trigger => "trigger",
                values::ValueType::Bool => "bool",
                values::ValueType::Integer => "integer",
                values::ValueType::Decimal => "decimal",
                values::ValueType::Color => "color",
                values::ValueType::IntegerRange => "range<integer>",
                values::ValueType::DecimalRange => "range<decimal>",
                values::ValueType::ColorRange => "range<color>",
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

impl<R: Ref<Node>, I: Iterator<Item = R>> From<I> for ListOutput<NodeName> {
    fn from(iter: I) -> Self {
        return Self {
            elements: iter.map(|node| NodeName(node.name().to_string())).collect(),
        };
    }
}

impl<R: Ref<Input>, I: Iterator<Item = R>> From<I> for ListOutput<InputName> {
    fn from(iter: I) -> Self {
        return Self {
            elements: iter.map(|input| InputName(input.name().to_string())).collect(),
        };
    }
}

#[derive(Template)]
#[template(path = "node.txt")]
pub struct NodeOutput {
    name: NodeId,
    kind: String,
    nodes: HashMap<String, NodeName>,
    attrs: Vec<AttrName>,
}

impl From<Node> for NodeOutput {
    fn from(node: Node) -> Self {
        return Self {
            name: node.name().clone(),
            kind: node.kind().to_owned(),
            nodes: node.nodes().iter().map(|(key, node)| (key.clone(), NodeName(node.name().to_string()))).collect(),
            attrs: node.attributes().keys().cloned().map(|key| AttrName(key)).collect(),
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

impl From<Attribute> for AttributeOutput {
    fn from(attr: Attribute) -> Self {
        return Self {
            node: NodeName(attr.name().node().to_string()),
            path: attr.name().path().into_iter().cloned().map(AttrName).collect(),
            kind: attr.kind().to_owned(),
            value_type: ValueType(attr.value_type().to_owned()),
            attrs: attr.attrs().keys().cloned().map(AttrName).collect(),
        };
    }
}

#[derive(Template)]
#[template(path = "input.txt")]
pub struct InputOutput {
    name: InputName,
    value_type: ValueType,
}

impl From<Input> for InputOutput {
    fn from(input: Input) -> Self {
        return Self {
            name: InputName(input.name().to_string()),
            value_type: ValueType::from(input.value_type()),
        };
    }
}
