use itertools::Itertools;
use std::fmt;

use yansi::{Paint, Style};

use photonic_interface_grpc_client::attr::Attr;
use photonic_interface_grpc_client::input::Input;
use photonic_interface_grpc_client::node::{Node, NodeId};
use photonic_interface_grpc_client::{values, AttrId, InputId};

pub trait Output {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result;

    fn display(&self) -> impl fmt::Display {
        return Render(self);
    }
}

struct Render<'s, O>(&'s O)
where O: Output + ?Sized;
impl<O: Output + ?Sized> fmt::Display for Render<'_, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return self.0.render(f);
    }
}

const NODE_STYLE: Style = Style::new().bright_yellow();
const ATTR_STYLE: Style = Style::new().bright_cyan();
const INPUT_STYLE: Style = Style::new().bright_magenta();

const TYPE_STYLE: Style = Style::new().bright_blue();

impl Output for NodeId {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        return write!(f, "{}", self.as_ref().paint(NODE_STYLE));
    }
}

impl Output for AttrId {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        let path = self.path().iter().map(|e| e.paint(ATTR_STYLE)).join("/");

        return write!(f, "{}/{}", self.node().display(), path);
    }
}

impl Output for InputId {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        return write!(f, "{}", self.as_ref().paint(INPUT_STYLE));
    }
}

impl Output for values::ValueType {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        let value = match self {
            values::ValueType::Trigger => "trigger",
            values::ValueType::Bool => "bool",
            values::ValueType::Integer => "integer",
            values::ValueType::Decimal => "decimal",
            values::ValueType::Color => "color",
            values::ValueType::IntegerRange => "range<integer>",
            values::ValueType::DecimalRange => "range<decimal>",
            values::ValueType::ColorRange => "range<color>",
        };

        return write!(f, "{}", value.paint(TYPE_STYLE));
    }
}

impl Output for () {
    fn render(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        return Ok(());
    }
}

impl Output for Node {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "Node: {}", self.name().display())?;
        writeln!(f, "  Kind: {}", self.kind())?;
        writeln!(f, "  Nodes:")?;
        for (key, node) in self.nodes() {
            writeln!(f, "    - {}: {}", key, node.display())?;
        }
        writeln!(f, "  Attributes:")?;
        for attr in self.attrs() {
            writeln!(f, "    - {}", attr.display())?;
        }

        return Ok(());
    }
}

impl Output for Attr {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "Attribute: {}", self.name().display())?;
        writeln!(f, "  Kind: {}", self.kind())?;
        writeln!(f, "  Value Type: {}", self.value_type())?;
        writeln!(f, "  Attributes:")?;
        for attr in self.attrs() {
            writeln!(f, "    - {}", attr.display())?;
        }
        writeln!(f, "  Inputs:")?;
        for (key, input) in self.inputs() {
            writeln!(f, "    - {}: {}", key, input.display())?;
        }

        return Ok(());
    }
}

impl Output for Input {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "Input: {}", self.name().display())?;
        writeln!(f, "  Value Type: {}", self.value_type().display())?;

        return Ok(());
    }
}

pub struct ListOutput<T: Output> {
    elements: Vec<T>,
}

impl<T: Output> Output for ListOutput<T> {
    fn render(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        for element in &self.elements {
            writeln!(f, "- {}", element.display())?;
        }

        return Ok(());
    }
}

impl<I: Iterator<Item = NodeId>> From<I> for ListOutput<NodeId> {
    fn from(iter: I) -> Self {
        return Self {
            elements: iter.collect(),
        };
    }
}

impl<I: Iterator<Item = InputId>> From<I> for ListOutput<InputId> {
    fn from(iter: I) -> Self {
        return Self {
            elements: iter.collect(),
        };
    }
}
