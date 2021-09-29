use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use crossbeam::atomic::AtomicCell;

use crate::attr::{Attr, AttrValue, BoundAttrDecl, Bounded, Bounds, UnboundAttrDecl, Update};
use crate::scene::{AttrBuilder, InputHandle};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InputValueType {
    Trigger,
    Boolean,
    Integer,
    Decimal,
}

pub trait InputValue: Send + Copy + 'static {
    const TYPE: InputValueType;

    fn sender(sink: Sink<Self>) -> InputSender;
}

pub enum Poll<V>
where
    V: InputValue,
{
    Pending,
    Ready(V),
}

pub struct Input<V>
where
    V: InputValue,
{
    value: Arc<AtomicCell<Poll<V>>>,
}

#[derive(Clone)]
pub struct Sink<V>
where
    V: InputValue,
{
    value: Arc<AtomicCell<Poll<V>>>,
}

impl<V> Input<V>
where
    V: InputValue,
{
    pub fn poll(&mut self) -> Poll<V> {
        return self.value.swap(Poll::Pending);
    }

    pub fn sink(&self) -> Sink<V> {
        return Sink {
            value: self.value.clone(),
        };
    }
}

impl<V> Default for Input<V>
where
    V: InputValue,
{
    fn default() -> Self {
        return Self {
            value: Arc::new(AtomicCell::new(Poll::Pending)),
        };
    }
}

impl<V> Sink<V>
where
    V: InputValue,
{
    pub fn send(&self, next: V) {
        self.value.store(Poll::Ready(next));
    }
}

impl InputValue for () {
    const TYPE: InputValueType = InputValueType::Trigger;

    fn sender(sink: Sink<Self>) -> InputSender {
        return InputSender::Trigger(sink);
    }
}

impl InputValue for bool {
    const TYPE: InputValueType = InputValueType::Boolean;

    fn sender(sink: Sink<Self>) -> InputSender {
        return InputSender::Boolean(sink);
    }
}

impl InputValue for i64 {
    const TYPE: InputValueType = InputValueType::Integer;

    fn sender(sink: Sink<Self>) -> InputSender {
        return InputSender::Integer(sink);
    }
}

impl InputValue for f64 {
    const TYPE: InputValueType = InputValueType::Decimal;

    fn sender(sink: Sink<Self>) -> InputSender {
        return InputSender::Decimal(sink);
    }
}

pub enum InputSender {
    Trigger(Sink<()>),
    Boolean(Sink<bool>),
    Integer(Sink<i64>),
    Decimal(Sink<f64>),
}

impl std::fmt::Debug for InputSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "({})",
            match self {
                Self::Trigger(_) => "trigger",
                Self::Boolean(_) => "boolean",
                Self::Integer(_) => "integer",
                Self::Decimal(_) => "decimal",
            }
        );
    }
}

impl<V> InputHandle<V>
where
    V: InputValue + AttrValue,
{
    pub fn attr(self, initial: V) -> InputAttrDecl<V> {
        return InputAttrDecl {
            input: self,
            initial,
        };
    }
}

pub struct InputAttrDecl<V>
where
    V: InputValue + AttrValue,
{
    input: InputHandle<V>,
    initial: V,
}

pub struct BoundInputAttr<V>
where
    V: AttrValue + InputValue + Bounded,
{
    bounds: Bounds<V>,

    input: Input<V>,
    current: V,
}

impl<V> Attr for BoundInputAttr<V>
where
    V: AttrValue + InputValue + Bounded,
{
    type Element = V;
    const KIND: &'static str = "input";

    fn get(&self) -> Self::Element {
        self.current
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        if let Poll::Ready(update) = self.input.poll() {
            if let Ok(update) = self.bounds.ensure(update) {
                self.current = update;
                return Update::Changed(self.current);
            } else {
                return Update::Idle(self.current);
            }
        } else {
            return Update::Idle(self.current);
        }
    }
}

impl<V> BoundAttrDecl for InputAttrDecl<V>
where
    V: AttrValue + InputValue + Bounded,
{
    type Element = V;
    type Target = BoundInputAttr<V>;

    fn materialize(self, bounds: Bounds<V>, builder: &mut AttrBuilder) -> Result<Self::Target> {
        let input = builder.input("input", self.input)?;

        let initial = bounds.ensure(self.initial)?;

        return Ok(Self::Target {
            bounds,
            input,
            current: initial,
        });
    }
}

pub struct UnboundInputAttr<V>
where
    V: AttrValue + InputValue,
{
    input: Input<V>,
    current: V,
}

impl<V> Attr for UnboundInputAttr<V>
where
    V: AttrValue + InputValue,
{
    type Element = V;

    const KIND: &'static str = "manual";

    fn get(&self) -> Self::Element {
        self.current
    }

    fn update(&mut self, _duration: Duration) -> Update<V> {
        if let Poll::Ready(update) = self.input.poll() {
            self.current = update;
            return Update::Changed(self.current);
        } else {
            return Update::Idle(self.current);
        }
    }
}

impl<V> UnboundAttrDecl for InputAttrDecl<V>
where
    V: AttrValue + InputValue,
{
    type Element = V;
    type Target = UnboundInputAttr<V>;

    fn materialize(self, builder: &mut AttrBuilder) -> Result<Self::Target> {
        let input = builder.input("value", self.input)?;

        return Ok(Self::Target {
            input,
            current: self.initial,
        });
    }
}
