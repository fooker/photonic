use photonic::values::*;
use photonic::color::Black;
use photonic::core::*;
use photonic::inspection::*;
use photonic::math::Lerp;
use std::time::Duration;

struct SwitchRenderer<'a> {
    source1: Box<Renderer + 'a>,
    source2: Box<Renderer + 'a>,

    blend: f64,
}

impl<'a> Renderer for SwitchRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let source1 = self.source1.get(index);
        let source2 = self.source2.get(index);

        // TODO: Blending modes
        return MainColor::lerp(source1,
                               source2,
                               self.blend);
    }
}

pub struct SwitchNode {
    sources: Vec<Box<Node>>,

    position: IntValue,
}

impl SwitchNode {
    const CLASS: &'static str = "switch";

    pub fn new(sources: Vec<Box<Node>>,
               position: IntValueFactory) -> Result<Self, String> {
        let max = sources.len() as i64 - 1; // FIXME: Ugly?

        Ok(Self {
            sources,
            position: position(IntValueDecl{name: "position", min: Some(0), max: Some(max)})?,
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
        let position = self.position.get();
        // FIXME: Blending

//        let (position, blend) = (position.trunc() as usize, position.fract());
//
//        if blend != 0f64 {
//            return Box::new(SwitchRenderer {
//                source1: self.sources[(position + 0) % self.sources.len()].render(),
//                source2: self.sources[(position + 1) % self.sources.len()].render(),
//                blend,
//            });
//        } else {
//            return self.sources[position % self.sources.len()].render();
//        }

        return self.sources[position as usize % self.sources.len()].render();
    }
}

impl Dynamic for SwitchNode {
    fn update(&mut self, duration: &Duration) {
        for source in self.sources.iter_mut() {
            source.update(duration);
        }

        self.position.update(duration);
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
