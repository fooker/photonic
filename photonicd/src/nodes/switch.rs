use photonic::animation::*;
use photonic::color::Black;
use photonic::core::*;
use photonic::inspection::*;
use photonic::math::Lerp;
use photonic::values::*;
use std::time::Duration;

struct SwitchRenderer<'a> {
    source: Box<Renderer + 'a>,
    target: Box<Renderer + 'a>,

    blend: f64,
}

impl<'a> Renderer for SwitchRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let source = self.source.get(index);
        let target = self.target.get(index);

        // TODO: Blending modes
        return MainColor::lerp(source,
                               target,
                               self.blend);
    }
}

pub struct SwitchNode {
    sources: Vec<Box<Node>>,

    easing: Option<Easing>,

    position: IntValue,

    source: usize,
    target: usize,
    blend: f64,

    transition: Animation,
}

impl SwitchNode {
    const CLASS: &'static str = "switch";

    pub fn new(sources: Vec<Box<Node>>,
               easing: Option<Easing>,
               position: IntValueFactory) -> Result<Self, String> {
        let max = sources.len() as i64 - 1; // FIXME: Ugly?

        Ok(Self {
            sources,
            easing,
            position: position(IntValueDecl { name: "position", min: Some(0), max: Some(max) })?,  // TODO: Rework rules for minimum and maximum? Should be fixed here...
            source: 0,
            target: 0,
            blend: 0.0,
            transition: Animation::Idle,
        })
    }
}

impl Node for SwitchNode {
    fn class(&self) -> &'static str {
        Self::CLASS
    }
}

impl Source for SwitchNode {
    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        if self.source == self.target {
            return self.sources[self.source].render();
        } else {
            return Box::new(SwitchRenderer {
                source: self.sources[self.source].render(),
                target: self.sources[self.target].render(),
                blend: self.blend,
            });
        }
    }
}

impl Dynamic for SwitchNode {
    fn update(&mut self, duration: &Duration) {
        for source in self.sources.iter_mut() {
            source.update(duration);
        }

        if let Some(position) = self.position.update(duration) {
            let position = position as usize; // FIXME: Not sure
            if let Some(easing) = self.easing {
                self.source = self.target;
                self.target = position;
                self.blend = 0.0;
                self.transition = Animation::start(easing, 0.0, 1.0);
            } else {
                self.source = position;
                self.target = position;
            }
        }

        if let Some(value) = self.transition.update(duration) {
            self.blend = value;
        } else {
            self.source = self.target;
            self.blend = 0.0;
        }
    }
}

impl Inspection for SwitchNode {
    fn children(&self) -> Vec<NodeRef> {
        return self.sources.iter().map(|source| {
            NodeRef { name: "source", ptr: source.as_ref() }
        }).collect();
    }

    fn values(&self) -> Vec<ValueRef> {
        vec![
            ValueRef { name: "position", ptr: ValuePtr::Int(&self.position) }
        ]
    }
}
