use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Scene {
    pub size: usize,

    pub root: Node,
    pub output: Output,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Node {
    pub name: String,

    #[serde(alias = "type")]
    pub kind: String,

    #[serde(flatten)]
    pub config: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Input {
    pub input: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Output {
    #[serde(alias = "type")]
    pub kind: String,

    #[serde(flatten)]
    pub config: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Attr {
    Attr {
        #[serde(alias = "type")]
        kind: String,

        #[serde(flatten)]
        config: Value,
    },

    Input {
        #[serde(flatten)]
        input: Input,

        initial: Value,
    },

    Fixed(Value)
}