use crate::attr::Range;
use palette::rgb::Rgb;

use super::sink::{InputSink, Sink};
use super::{InputValue, InputValueType};

impl super::private::Sealed for () {}
impl InputValue for () {
    const TYPE: InputValueType = InputValueType::Trigger;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Trigger(sink);
    }
}

impl super::private::Sealed for bool {}
impl InputValue for bool {
    const TYPE: InputValueType = InputValueType::Boolean;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Boolean(sink);
    }
}

impl super::private::Sealed for i64 {}
impl InputValue for i64 {
    const TYPE: InputValueType = InputValueType::Integer;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Integer(sink);
    }
}

impl super::private::Sealed for f32 {}
impl InputValue for f32 {
    const TYPE: InputValueType = InputValueType::Decimal;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Decimal(sink);
    }
}

impl super::private::Sealed for Rgb {}
impl InputValue for Rgb {
    const TYPE: InputValueType = InputValueType::Color;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Color(sink);
    }
}

impl super::private::Sealed for Range<i64> {}
impl InputValue for Range<i64> {
    const TYPE: InputValueType = InputValueType::Integer;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::IntegerRange(sink);
    }
}

impl super::private::Sealed for Range<f32> {}
impl InputValue for Range<f32> {
    const TYPE: InputValueType = InputValueType::Decimal;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::DecimalRange(sink);
    }
}

impl super::private::Sealed for Range<Rgb> {}
impl InputValue for Range<Rgb> {
    const TYPE: InputValueType = InputValueType::Color;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::ColorRange(sink);
    }
}
