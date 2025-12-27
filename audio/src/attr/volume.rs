use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Stream, StreamConfig};

use photonic::attr::Bounds;
use photonic::math::Lerp;
use photonic::{Attr, AttrBuilder, BoundAttrDecl, FreeAttrDecl, RenderContext};

pub struct Volume {}

impl Volume {
    fn stream(f: impl Fn(&[f32]) + Send + 'static) -> Result<Stream> {
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
}

impl FreeAttrDecl<f32> for Volume {
    const KIND: &'static str = "volume";

    type Attr = VolumeAttr;

    fn materialize(self, _builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let last = Arc::new(Mutex::new(0.0));
        let next = last.clone();

        let stream = Self::stream(move |data| {
            let value = data.iter().copied().fold(f32::MIN, f32::max);
            let value = value.clamp(0.0, 1.0);

            *next.lock().expect("Lock") = value;
        })?;

        return Ok(Self::Attr {
            stream,
            last,
        });
    }
}

impl BoundAttrDecl<f32> for Volume {
    const KIND: &'static str = "volume";

    type Attr = VolumeAttr;

    fn materialize(self, bounds: Bounds<f32>, _builder: &mut AttrBuilder) -> Result<Self::Attr> {
        let last = Arc::new(Mutex::new(0.0));
        let next = last.clone();

        let stream = Self::stream(move |data| {
            let value = data.iter().copied().fold(f32::MIN, f32::max);
            let value = value.clamp(0.0, 1.0);
            let value = Lerp::lerp(bounds.min, bounds.max, value);

            *next.lock().expect("Lock") = value;
        })?;

        return Ok(Self::Attr {
            stream,
            last,
        });
    }
}

pub struct VolumeAttr {
    #[allow(unused)]
    stream: Stream,

    last: Arc<Mutex<f32>>,
}

impl Attr<f32> for VolumeAttr {
    fn update(&mut self, _ctx: &RenderContext) -> f32 {
        let last = self.last.lock().expect("Lock");

        return *last;
    }
}
