use failure::Error;

use crate::node::Render;

pub trait OutputDecl {
    type Element;
    type Target: Output<Element=Self::Element>;

    fn materialize(self, size: usize) -> Result<Self::Target, Error>
        where Self::Target: std::marker::Sized;
}

pub trait Output {
    type Element;

    const KIND: &'static str;

    fn render(&mut self, render: &dyn Render<Element=Self::Element>);
}