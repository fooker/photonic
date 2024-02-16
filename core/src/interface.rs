use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::Arc;

use anyhow::Result;

use crate::attr::AttrValueType;
use crate::input::{InputSink, InputValueType};

#[derive(Debug)]
pub struct NodeInfo {
    pub kind: &'static str,

    pub name: String,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub attrs: HashMap<String, Arc<AttrInfo>>,
}

#[derive(Debug)]
pub struct AttrInfo {
    pub kind: &'static str,

    pub value_type: AttrValueType,

    pub attrs: HashMap<String, Arc<AttrInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

#[derive(Debug)]
pub struct InputInfo {
    pub name: String,
    pub value_type: InputValueType,

    pub sink: InputSink,
}

pub struct Introspection {
    pub root: Arc<NodeInfo>,

    pub nodes: HashMap<String, Arc<NodeInfo>>,
    pub inputs: HashMap<String, Arc<InputInfo>>,
}

impl Introspection {
    pub fn with(root: Arc<NodeInfo>) -> Arc<Self> {
        let nodes: HashMap<_, _> =
            TreeIter::new(&root, |node| node.nodes.values()).map(|node| (node.name.clone(), node.clone())).collect();

        let inputs = TreeIter::new(&root, |node| node.nodes.values())
            .flat_map(|node| node.attrs.values())
            .flat_map(|attr| TreeIter::new(attr, |attr| attr.attrs.values()))
            .flat_map(|attr| attr.inputs.values())
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

            for (name, attr) in &attr.attrs {
                log_attr(depth + 1, name, attr);
            }

            for (name, input) in &attr.inputs {
                log_input(depth + 1, name, input);
            }

            eprintln!("🔎  {}}}", indent);
        }

        fn log_node(depth: usize, key: &str, node: &NodeInfo) {
            let indent = String::from("  ").repeat(depth);
            eprintln!("🔎  {}⭐ {} = {}@{} {{", indent, key, node.kind, node.name);

            for (name, attr) in &node.attrs {
                log_attr(depth + 1, name, attr);
            }

            for (name, node) in &node.nodes {
                log_node(depth + 1, name, node);
            }

            eprintln!("🔎  {}}}", indent)
        }

        log_node(0, "root", &self.root);
    }
}

pub trait Interface {
    fn listen(self, introspection: Arc<Introspection>) -> impl Future<Output = Result<()>> + Send;
}

struct TreeIter<'a, T, F, I>
where
    T: 'a,
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a T>,
{
    f: F,
    queue: VecDeque<&'a T>,
}

impl<'a, T, F, I> TreeIter<'a, T, F, I>
where
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a T>,
{
    pub fn new(t: &'a T, f: F) -> Self {
        return Self {
            f,
            queue: VecDeque::from(vec![t]),
        };
    }
}

impl<'a, T, F, I> Iterator for TreeIter<'a, T, F, I>
where
    F: Fn(&'a T) -> I,
    I: Iterator<Item = &'a T>,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.queue.pop_front() {
            self.queue.extend((self.f)(t));
            return Some(t);
        } else {
            return None;
        }
    }
}
