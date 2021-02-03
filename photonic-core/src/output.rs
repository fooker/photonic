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

    fn render<E: Into<Self::Element>>(&mut self, render: &dyn Render<Element=E>);
}