use serde::Deserialize;

use photonic::AttrValue;

pub type Anything = serde_value::Value;

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
    pub config: Anything,
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
    pub config: Anything,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
// TODO: Life the requirement of V: Deserialize and make fixed values optional by type
pub enum Attr<V>
where V: AttrValue
{
    Attr {
        #[serde(alias = "type")]
        kind: String,

        #[serde(flatten)]
        config: Anything,
    },

    Input {
        #[serde(flatten)]
        input: Input,

        initial: V,
    },

    Fixed(V),
}
