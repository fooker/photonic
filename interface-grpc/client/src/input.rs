use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use tonic::transport::Channel;

use photonic_interface_grpc_proto::interface_client::InterfaceClient;
use photonic_interface_grpc_proto::{input_value, InputInfoResponse, InputSendRequest, InputValue, InputValueType};

use crate::values::{ColorValue, RangeValue, ValueType};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct InputId(pub(crate) String);

impl fmt::Display for InputId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for InputId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for InputId {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl FromStr for InputId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self(s.to_owned()));
    }
}

pub struct Input {
    client: Arc<Mutex<InterfaceClient<Channel>>>,

    name: InputId,

    value_type: ValueType,
}

impl Input {
    pub(crate) fn from_input_info(client: Arc<Mutex<InterfaceClient<Channel>>>, info: InputInfoResponse) -> Self {
        let value_type = match info.value_type() {
            InputValueType::Trigger => ValueType::Trigger,
            InputValueType::Bool => ValueType::Bool,
            InputValueType::Integer => ValueType::Integer,
            InputValueType::Decimal => ValueType::Decimal,
            InputValueType::Color => ValueType::Color,
            InputValueType::IntegerRange => ValueType::IntegerRange,
            InputValueType::DecimalRange => ValueType::DecimalRange,
            InputValueType::ColorRange => ValueType::ColorRange,
        };

        Self {
            client,
            name: InputId(info.name),
            value_type,
        }
    }

    pub fn name(&self) -> &InputId {
        return &self.name;
    }

    pub fn value_type(&self) -> ValueType {
        return self.value_type;
    }

    pub fn sink(&self) -> InputSink {
        return match self.value_type {
            ValueType::Trigger => InputSink::Trigger(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::Bool => InputSink::Boolean(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::Integer => InputSink::Integer(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::Decimal => InputSink::Decimal(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::Color => InputSink::Color(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::IntegerRange => InputSink::IntegerRange(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::DecimalRange => InputSink::DecimalRange(Sink {
                input: self,
                value_type: PhantomData,
            }),
            ValueType::ColorRange => InputSink::ColorRange(Sink {
                input: self,
                value_type: PhantomData,
            }),
        };
    }
}

pub enum InputSink<'i> {
    Trigger(Sink<'i, ()>),
    Boolean(Sink<'i, bool>),
    Integer(Sink<'i, i64>),
    Decimal(Sink<'i, f32>),
    Color(Sink<'i, ColorValue>),
    IntegerRange(Sink<'i, RangeValue<i64>>),
    DecimalRange(Sink<'i, RangeValue<f32>>),
    ColorRange(Sink<'i, RangeValue<ColorValue>>),
}

pub struct Sink<'i, V> {
    input: &'i Input,
    value_type: PhantomData<V>,
}

impl Sink<'_, ()> {
    pub async fn trigger(&self) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::Trigger(())),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, bool> {
    pub async fn send(&self, value: bool) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::Bool(value)),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, i64> {
    pub async fn send(&self, value: i64) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::Integer(value)),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, f32> {
    pub async fn send(&self, value: f32) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::Decimal(value)),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, ColorValue> {
    pub async fn send(&self, value: ColorValue) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::Color(value.into())),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, RangeValue<i64>> {
    pub async fn send(&self, value: RangeValue<i64>) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::IntegerRange(value.into())),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, RangeValue<f32>> {
    pub async fn send(&self, value: RangeValue<f32>) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::DecimalRange(value.into())),
                }),
            })
            .await?;
        return Ok(());
    }
}

impl Sink<'_, RangeValue<ColorValue>> {
    pub async fn send(&self, value: RangeValue<ColorValue>) -> Result<()> {
        let mut client = self.input.client.lock_arc();

        client
            .input_send(InputSendRequest {
                name: self.input.name.0.clone(),
                value: Some(InputValue {
                    value: Some(input_value::Value::ColorRange(value.into())),
                }),
            })
            .await?;
        return Ok(());
    }
}
