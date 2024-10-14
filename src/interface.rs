use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::sync::{Arc, OnceLock};

use anyhow::Result;
use futures::Stream;

use crate::attr::AttrValueType;
use crate::input::{AnyInputValue, InputSink, InputValueType};
use crate::utils::TreeIterator;

#[derive(Debug)]
pub struct NodeInfoBuilder {
    pub key: String,

    pub kind: &'static str,
    pub name: String,

    pub nodes: HashMap<String, NodeInfoBuilder>,
    pub attrs: HashMap<String, AttrInfoBuilder>,
}

#[derive(Debug)]
pub struct AttrInfoBuilder {
    pub key: String,

    pub kind: &'static str,
    pub value_type: AttrValueType,

    pub attrs: HashMap<String, AttrInfoBuilder>,
    pub inputs: HashMap<String, InputInfoBuilder>,
}

#[derive(Debug)]
pub struct InputInfoBuilder {
    pub key: String,

    pub name: String,
    pub value_type: InputValueType,

    pub sink: InputSink,
}

#[derive(Debug)]
pub struct NodeInfo {
    key: String,

    kind: &'static str,
    name: String,

    parent: Option<Arc<NodeInfo>>,

    nodes: OnceLock<HashMap<String, Arc<NodeInfo>>>,
    attrs: OnceLock<HashMap<String, Arc<AttrInfo>>>,
}

impl NodeInfo {
    pub fn key(&self) -> &str {
        return &self.key;
    }

    pub fn kind(&self) -> &'static str {
        return self.kind;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn parent(&self) -> Option<&Arc<NodeInfo>> {
        return self.parent.as_ref();
    }

    pub fn nodes(&self) -> &HashMap<String, Arc<NodeInfo>> {
        return self.nodes.get().expect("set");
    }

    pub fn attrs(&self) -> &HashMap<String, Arc<AttrInfo>> {
        return self.attrs.get().expect("set");
    }

    pub fn find_attr<'i, I, Q>(&self, mut path: I) -> Option<&AttrInfo>
    where
        I: Iterator<Item = &'i Q>,
        Q: Hash + Eq + 'i,
        String: Borrow<Q>,
    {
        return match path.next() {
            None => None,
            Some(attr) => self.attrs.get().expect("set").get(attr)?.find_attr(path),
        };
    }
}

#[derive(Debug)]
pub struct AttrInfo {
    key: String,

    kind: &'static str,
    value_type: AttrValueType,

    node: Arc<NodeInfo>,
    parent: Option<Arc<AttrInfo>>,

    attrs: OnceLock<HashMap<String, Arc<AttrInfo>>>,
    inputs: OnceLock<HashMap<String, Arc<InputInfo>>>,
}

impl AttrInfo {
    pub fn key(&self) -> &str {
        return &self.key;
    }

    pub fn kind(&self) -> &'static str {
        return self.kind;
    }

    pub fn value_type(&self) -> AttrValueType {
        return self.value_type;
    }

    pub fn node(&self) -> &Arc<NodeInfo> {
        return &self.node;
    }

    pub fn parent(&self) -> Option<&Arc<AttrInfo>> {
        return self.parent.as_ref();
    }

    pub fn find_attr<'i, I, Q>(&self, mut path: I) -> Option<&Self>
    where
        I: Iterator<Item = &'i Q>,
        Q: Hash + Eq + 'i,
        String: Borrow<Q>,
    {
        return match path.next() {
            None => Some(self),
            Some(attr) => self.attrs.get().expect("set").get(attr)?.find_attr(path),
        };
    }

    pub fn attrs(&self) -> &HashMap<String, Arc<AttrInfo>> {
        return self.attrs.get().expect("set");
    }

    pub fn inputs(&self) -> &HashMap<String, Arc<InputInfo>> {
        return self.inputs.get().expect("set");
    }
}

#[derive(Debug)]
pub struct InputInfo {
    key: String,

    name: String,
    value_type: InputValueType,

    node: Arc<NodeInfo>,
    attr: Arc<AttrInfo>,

    sink: InputSink,
}

impl InputInfo {
    pub fn key(&self) -> &str {
        return &self.key;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn value_type(&self) -> InputValueType {
        return self.value_type;
    }

    pub fn node(&self) -> &Arc<NodeInfo> {
        return &self.node;
    }

    pub fn attr(&self) -> &Arc<AttrInfo> {
        return &self.attr;
    }

    pub fn sink(&self) -> &InputSink {
        return &self.sink;
    }

    pub fn subscribe(&self) -> impl Stream<Item = AnyInputValue> + Send + Unpin {
        return self.sink.subscribe();
    }
}

pub struct Introspection {
    pub root: Arc<NodeInfo>,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

impl Introspection {
    pub fn with(root: NodeInfoBuilder) -> Arc<Self> {
        fn build_input(builder: InputInfoBuilder, node: Arc<NodeInfo>, attr: Arc<AttrInfo>) -> Arc<InputInfo> {
            return Arc::new(InputInfo {
                key: builder.key,
                name: builder.name,
                value_type: builder.value_type,
                node,
                attr,
                sink: builder.sink,
            });
        }

        fn build_attr(builder: AttrInfoBuilder, node: Arc<NodeInfo>, parent: Option<Arc<AttrInfo>>) -> Arc<AttrInfo> {
            let result = Arc::new(AttrInfo {
                key: builder.key,
                kind: builder.kind,
                value_type: builder.value_type,
                node: node.clone(),
                parent,
                attrs: OnceLock::new(),
                inputs: OnceLock::new(),
            });

            result
                .attrs
                .set(
                    builder
                        .attrs
                        .into_iter()
                        .map(|(key, attr)| (key, build_attr(attr, node.clone(), Some(result.clone()))))
                        .collect(),
                )
                .expect("unset");

            result
                .inputs
                .set(
                    builder
                        .inputs
                        .into_iter()
                        .map(|(key, input)| (key, build_input(input, node.clone(), result.clone())))
                        .collect(),
                )
                .expect("unset");

            return result;
        }

        fn build_node(builder: NodeInfoBuilder, parent: Option<Arc<NodeInfo>>) -> Arc<NodeInfo> {
            let result = Arc::new(NodeInfo {
                key: builder.key,
                kind: builder.kind,
                name: builder.name,
                parent,
                nodes: OnceLock::new(),
                attrs: OnceLock::new(),
            });

            result
                .nodes
                .set(
                    builder
                        .nodes
                        .into_iter()
                        .map(|(key, node)| (key, build_node(node, Some(result.clone()))))
                        .collect(),
                )
                .expect("unset");

            result
                .attrs
                .set(
                    builder
                        .attrs
                        .into_iter()
                        .map(|(key, attr)| (key, build_attr(attr, result.clone(), None)))
                        .collect(),
                )
                .expect("unset");

            return result;
        }

        let root = build_node(root, None);

        let nodes = TreeIterator::new(&root, |node| node.nodes().values())
            .map(|node| (node.name.clone(), node.clone()))
            .collect::<HashMap<_, _>>();

        let inputs = TreeIterator::new(&root, |node| node.nodes().values())
            .flat_map(|node| node.attrs().values())
            .flat_map(|attr| TreeIterator::new(attr, |attr| attr.attrs().values()))
            .flat_map(|attr| attr.inputs().values())
            .map(|input| (input.name.clone(), input.clone()))
            .collect();

        return Arc::new(Self {
            root,
            nodes,
            inputs,
        });
    }

    pub fn log(&self) {
        eprintln!("🔎 Full scene:");

        fn log_input(depth: usize, key: &str, input: &InputInfo) {
            let indent = String::from("  ").repeat(depth);
            eprintln!("🔎  {}🎛 {} = {}", indent, key, input.name);
        }

        fn log_attr(depth: usize, key: &str, attr: &AttrInfo) {
            let indent = String::from("  ").repeat(depth);
            eprintln!("🔎  {}🪛 {} = {}:{} {{", indent, key, attr.value_type, attr.kind);

            for (name, attr) in attr.attrs() {
                log_attr(depth + 1, name, attr);
            }

            for (name, input) in attr.inputs() {
                log_input(depth + 1, name, input);
            }

            eprintln!("🔎  {}}}", indent);
        }

        fn log_node(depth: usize, key: &str, node: &NodeInfo) {
            let indent = String::from("  ").repeat(depth);
            eprintln!("🔎  {}⭐ {} = {}@{} {{", indent, key, node.kind, node.name);

            for (name, attr) in node.attrs() {
                log_attr(depth + 1, name, attr);
            }

            for (name, node) in node.nodes() {
                log_node(depth + 1, name, node);
            }

            eprintln!("🔎  {}}}", indent)
        }

        log_node(0, "root", &self.root);
    }
}

pub trait Interface {
    fn listen(self, introspection: Arc<Introspection>) -> impl Future<Output = Result<()>> + Send + 'static;
}
