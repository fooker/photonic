use std::collections::HashMap;

use crate::SendValue;
use anyhow::Error;
use async_trait::async_trait;
use serde::Serialize;

pub mod grpc;

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputValueType {
    Trigger,
    Boolean,
    Integer,
    Decimal,
}

#[derive(Serialize)]
pub struct InputInfo {
    pub name: String,
    pub kind: String,

    pub value_type: InputValueType,
}

#[derive(Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum AttrValueType {
    Boolean,
    Integer,
    Decimal,
    Color,
    Range(&'static AttrValueType),
}

#[derive(Serialize)]
pub struct AttrInfo {
    pub kind: String,

    pub value_type: AttrValueType,

    pub attrs: HashMap<String, AttrInfo>,
    pub inputs: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct NodeInfo {
    pub name: String,
    pub kind: String,

    pub nodes: HashMap<String, String>,
    pub attrs: HashMap<String, AttrInfo>,
}

#[async_trait]
pub trait Client: Sized {
    async fn connect(cfg: String) -> Result<Self, Error>;

    async fn nodes(&mut self) -> Result<Vec<String>, Error>;
    async fn node(&mut self, name: Option<String>) -> Result<Option<NodeInfo>, Error>;

    async fn send(&mut self, name: String, value: SendValue) -> Result<(), Error>;
}
