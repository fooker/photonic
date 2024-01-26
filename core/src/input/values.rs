use palette::rgb::Rgb;
use crate::attr::Range;

use super::{InputValue, InputValueType};
use super::sink::{InputSink, Sink};

impl InputValue for () {
    const TYPE: InputValueType = InputValueType::Trigger;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Trigger(sink);
    }
}

impl InputValue for bool {
    const TYPE: InputValueType = InputValueType::Boolean;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Boolean(sink);
    }
}

impl InputValue for i64 {
    const TYPE: InputValueType = InputValueType::Integer;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Integer(sink);
    }
}

impl InputValue for f32 {
    const TYPE: InputValueType = InputValueType::Decimal;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Decimal(sink);
    }
}

impl InputValue for Rgb {
    const TYPE: InputValueType = InputValueType::Color;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::Color(sink);
    }
}

impl InputValue for Range<i64> {
    const TYPE: InputValueType = InputValueType::Integer;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::IntegerRange(sink);
    }
}

impl InputValue for Range<f32> {
    const TYPE: InputValueType = InputValueType::Decimal;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::DecimalRange(sink);
    }
}

impl InputValue for Range<Rgb> {
    const TYPE: InputValueType = InputValueType::Color;
    fn sink(sink: Sink<Self>) -> InputSink {
        return InputSink::ColorRange(sink);
    }
}