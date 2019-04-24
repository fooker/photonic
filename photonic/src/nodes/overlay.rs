use std::time::Duration;

use crate::animation::Animation;
use crate::animation::Easing;
use crate::core::*;
use crate::math::Lerp;
use crate::values::*;
use failure::Error;

struct OverlayRenderer<'a> {
    base: Box<Renderer + 'a>,
    overlay: Box<Renderer + 'a>,

    blend: f64,
}

impl<'a> Renderer for OverlayRenderer<'a> {
    fn get(&self, index: usize) -> MainColor {
        let base = self.base.get(index);
        let overlay = self.overlay.get(index);

        // TODO: Blending modes
        return MainColor::lerp(base,
                               overlay,
                               self.blend);
    }
}

pub struct OverlayNodeDecl<Base: Node, Overlay: Node> {
    pub base: Base,
    pub overlay: Overlay,

    pub blend: Box<BoundValueDecl<f64>>,

    pub easing: Option<Easing>,
}

pub struct OverlayNode<Base: Node, Overlay: Node> {
    base: Base,
    overlay: Overlay,

    blend: Box<Value<f64>>,

    easing: Option<Easing>,
    transition: Animation,
}

impl<Base: Node, Overlay: Node> NodeDecl for OverlayNodeDecl<Base, Overlay> {
    type Node = OverlayNode<Base, Overlay>;

    fn new(self, size: usize) -> Result<Self::Node, Error> {
        return Ok(Self::Node {
            base: self.base,
            overlay: self.overlay,
            blend: self.blend.new(Bounds::norm())?,
            easing: self.easing,
            transition: Animation::Idle,
        });
    }
}

impl<Base: Node, Overlay: Node> Node for OverlayNode<Base, Overlay> {
    const TYPE: &'static str = "overlay";

    fn update(&mut self, duration: &Duration) {
        self.base.update(duration);
        self.overlay.update(duration);
        self.blend.update(duration);
    }

    fn render<'a>(&'a self) -> Box<Renderer + 'a> {
        let blend = self.blend.get();
        if blend > 0f64 {
            return Box::new(OverlayRenderer {
                base: self.base.render(),
                overlay: self.overlay.render(),
                blend,
            });
        } else {
            return self.base.render();
        }
    }
}

//impl Inspection for OverlayNode {
//    fn children(&self) -> Vec<NodeRef> {
//        vec![
//            NodeRef { name: "base", ptr: self.base.as_ref() },
//            NodeRef { name: "overlay", ptr: self.overlay.as_ref() },
//        ]
//    }
//
//    fn values(&self) -> Vec<ValueRef> {
//        vec![
////            ValueRef { name: "blend", ptr: self.blend },
//        ]
//    }
//}

