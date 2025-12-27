use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleRate, Stream, StreamConfig};

use photonic::attr::Bounds;
use photonic::math::Lerp;
use photonic::{Attr, AttrBuilder, BoundAttrDecl, FreeAttrDecl, RenderContext};

pub struct Power {
    cutoff_freq: Option<f32>,
}

impl Default for Power {
    fn default() -> Self {
        return Self::new();
    }
}

impl Power {
    pub fn new() -> Self {
        return Self {
            cutoff_freq: None,
        };
    }

    pub fn with_low_pass_filter(mut self, freq: f32) -> Self {
        self.cutoff_freq = Some(freq);
        return self;
    }
}

impl Power {
    fn stream(mut f: impl FnMut(&[f32]) + Send + 'static) -> anyhow::Result<Stream> {
        let host = cpal::default_host();
        let input = host.input_devices()?.nth(0).ok_or_else(|| anyhow!("No audio input device available"))?;

        let config = StreamConfig {
            channels: 1,
            sample_rate: 44100,
            buffer_size: BufferSize::Default,
        };

        let stream = input.build_input_stream(&config, move |data: &[f32], _| f(data), |_err| {}, None)?;

        stream.play()?;

        return Ok(stream);
    }

    fn low_pass(
        cutoff_freq: Option<f32>,
        sample_rate: SampleRate,
        mut f: impl FnMut(&[f32]) + Send + 'static,
    ) -> impl FnMut(&[f32]) {
        // https://en.wikipedia.org/wiki/Low-pass_filter#Simple_infinite_impulse_response_filter

        let alpha = cutoff_freq.map(|cutoff_freq| {
            let rc = 1.0 / (cutoff_freq * 2.0 * core::f32::consts::PI);
            let dt = 1.0 / sample_rate as f32;
            let alpha = dt / (rc + dt);
            return alpha;
        });

        let mut out = Vec::new();

        return move |data: &[f32]| {
            let Some(alpha) = alpha else {
                return f(data);
            };

            out.clear();
            out.reserve(data.len());

            // TODO: Can we do this without a copy - looking at you, cpal.
            out.push(data[0] * alpha);
            for i in 1..data.len() {
                out.push(out[i - 1] + alpha * (data[i] - out[i - 1]));
            }

            f(&out);
        };
    }

    fn analyzer(next: Arc<Mutex<f32>>, bounds: Bounds<f32>) -> impl FnMut(&[f32]) {
        let mut history = VecDeque::from([0.0; 128]);

        return move |data| {
            let power: f32 = data.iter().map(|v| v.powi(2)).sum::<f32>();

            let base: f32 = history.iter().sum::<f32>();
            let base = base / history.len() as f32;

            history.rotate_left(1);
            history.push_back(power);

            if base == 0.0 {
                return;
            }

            let value = (power - base) / base;
            let value = value.clamp(0.0, 1.0);
            let value = value.powi(2);
            let value = f32::lerp(bounds.min, bounds.max, value);

            *next.lock().expect("Lock") = value;
        };
    }
}

impl FreeAttrDecl<f32> for Power {
    const KIND: &'static str = "beat";

    type Attr = PowerAttr;

    fn materialize(self, _builder: &mut AttrBuilder) -> anyhow::Result<Self::Attr> {
        let last = Arc::new(Mutex::new(0.0));
        let next = last.clone();

        let stream = Self::low_pass(self.cutoff_freq, 44100, Self::analyzer(next, Bounds::normal()));
        let stream = Self::stream(stream)?;

        return Ok(Self::Attr {
            stream,
            last,
        });
    }
}

impl BoundAttrDecl<f32> for Power {
    const KIND: &'static str = "beat";

    type Attr = PowerAttr;

    fn materialize(self, bounds: Bounds<f32>, _builder: &mut AttrBuilder) -> anyhow::Result<Self::Attr> {
        let last = Arc::new(Mutex::new(0.0));
        let next = last.clone();

        let stream = Self::low_pass(self.cutoff_freq, 44100, Self::analyzer(next, bounds));

        let stream = Self::stream(stream)?;

        return Ok(Self::Attr {
            stream,
            last,
        });
    }
}

pub struct PowerAttr {
    #[allow(unused)]
    stream: Stream,

    last: Arc<Mutex<f32>>,
}

impl Attr<f32> for PowerAttr {
    fn update(&mut self, _ctx: &RenderContext) -> f32 {
        let last = self.last.lock().expect("Lock");

        return *last;
    }
}
