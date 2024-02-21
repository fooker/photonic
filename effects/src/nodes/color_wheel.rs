use anyhow::Result;
use palette::Hsv;

use photonic::{Buffer, Context, Node, NodeBuilder, NodeDecl};
use photonic_dyn::DynamicNode;

#[derive(DynamicNode)]
pub struct ColorWheel {
    pub scale: f32,
    pub speed: f32,
    pub offset: f32,

    pub saturation: f32,
    pub intensity: f32,
}

pub struct ColorWheelNode {
    scale: f32,
    speed: f32,
    offset: f32,

    saturation: f32,
    intensity: f32,

    position: f32,
}

impl NodeDecl for ColorWheel {
    type Node = ColorWheelNode;

    async fn materialize(self, _builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        return Ok(Self::Node {
            scale: self.scale,           // TODO: Make this an attr
            speed: self.speed,           // TODO: Make this an attr
            offset: self.offset,         // TODO: Make this an attr
            saturation: self.saturation, // TODO: Make this an attr
            intensity: self.intensity,   // TODO: Make this an attr
            position: 0.0,
        });
    }
}

impl Node for ColorWheelNode {
    const KIND: &'static str = "color_wheel";

    type Element = Hsv;

    fn update(&mut self, ctx: &Context, out: &mut Buffer<Self::Element>) -> Result<()> {
        self.position += ctx.duration.as_secs_f32() * self.speed;

        if self.scale <= 0.0 {
            let hue = (self.offset + self.position) * 360.0;
            out.fill(Hsv::new(hue, self.saturation, self.intensity))
        } else {
            for i in 0..out.len() {
                let hue = (i as f32 / out.len() as f32 * self.scale + self.offset + self.position) * 360.0;
                out[i] = Hsv::new(hue, self.saturation, self.intensity);
            }
        }

        return Ok(());
    }
}
