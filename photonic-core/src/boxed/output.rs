use anyhow::Error;

use crate::boxed::Wrap;
use crate::node::Render;
use crate::output::{Output, OutputDecl};

trait AsBoxedOutputDecl<Element> {
    fn materialize(self: Box<Self>, size: usize) -> Result<BoxedOutput<Element>, Error>;
}

impl<T, Element> AsBoxedOutputDecl<Element> for T
    where
        T: OutputDecl<Element=Element>,
        T::Target: 'static,
{
    fn materialize(self: Box<Self>, size: usize) -> Result<BoxedOutput<Element>, Error> {
        return T::materialize(*self, size).map(BoxedOutput::wrap);
    }
}

pub struct BoxedOutputDecl<Element> {
    decl: Box<dyn AsBoxedOutputDecl<Element>>,
}

impl<Element, Decl> Wrap<Decl> for BoxedOutputDecl<Element>
    where
        Decl: OutputDecl<Element=Element> + 'static,
{
    fn wrap(decl: Decl) -> Self {
        return Self {
            decl: Box::new(decl),
        };
    }
}

impl<Element> OutputDecl for BoxedOutputDecl<Element> {
    type Element = Element;
    type Target = BoxedOutput<Element>;

    fn materialize(self, size: usize) -> Result<Self::Target, Error>
        where
            Self::Target: Sized,
    {
        return self.decl.materialize(size);
    }
}

trait AsBoxedOutput<Element> {
    fn render(&mut self, render: &dyn Render<Element=Element>) -> Result<(), Error>;
}

impl<T, Element> AsBoxedOutput<Element> for T
    where
        T: Output<Element=Element>,
{
    fn render(&mut self, render: &dyn Render<Element=Element>) -> Result<(), Error> {
        return T::render(self, render);
    }
}

pub struct BoxedOutput<Element> {
    output: Box<dyn AsBoxedOutput<Element>>,
}

impl<Element, Output> Wrap<Output> for BoxedOutput<Element>
    where
        Output: self::Output<Element=Element> + 'static,
{
    fn wrap(output: Output) -> Self {
        return Self {
            output: Box::new(output),
        };
    }
}

impl<Element> Output for BoxedOutput<Element> {
    type Element = Element;

    const KIND: &'static str = "boxed";

    fn render(&mut self, render: &dyn Render<Element=Self::Element>) -> Result<(), Error> {
        return self.output.render(render);
    }
}
