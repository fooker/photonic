use anyhow::Error;

use crate::boxed::Wrap;
use crate::node::{Node, NodeDecl, RenderType};
use crate::output::{Output, OutputDecl};

trait AsBoxedOutputDecl<Node>
    where
        Node: self::NodeDecl,
{
    fn materialize(self: Box<Self>, size: usize) -> Result<BoxedOutput<Node::Target>, Error>;
}

impl<T, Node> AsBoxedOutputDecl<Node> for T
    where
        T: OutputDecl<Node>,
        T::Target: 'static,
        Node: self::NodeDecl,
{
    fn materialize(self: Box<Self>, size: usize) -> Result<BoxedOutput<Node::Target>, Error> {
        return T::materialize(*self, size).map(BoxedOutput::wrap);
    }
}

pub struct BoxedOutputDecl<Node>
    where
        Node: self::NodeDecl,
{
    decl: Box<dyn AsBoxedOutputDecl<Node>>,
}

impl<Node, Decl> Wrap<Decl> for BoxedOutputDecl<Node>
    where
        Node: self::NodeDecl + 'static,
        Decl: OutputDecl<Node> + 'static,
{
    fn wrap(decl: Decl) -> Self {
        return Self {
            decl: Box::new(decl),
        };
    }
}

impl<Node> OutputDecl<Node> for BoxedOutputDecl<Node>
    where
        Node: self::NodeDecl,
{
    type Target = BoxedOutput<Node::Target>;

    fn materialize(self, size: usize) -> Result<Self::Target, Error>
        where
            Self::Target: Sized,
    {
        return self.decl.materialize(size);
    }
}

trait AsBoxedOutput<Node>
    where
        Node: self::Node,
{
    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<(), Error>;
}

impl<T, Node> AsBoxedOutput<Node> for T
    where
        T: Output<Node>,
        Node: self::Node,
{
    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<(), Error> {
        return T::render(self, render);
    }
}

pub struct BoxedOutput<Node>
    where
        Node: self::Node,
{
    output: Box<dyn AsBoxedOutput<Node>>,
}

impl<Node, Output> Wrap<Output> for BoxedOutput<Node>
    where
        Node: self::Node,
        Output: self::Output<Node> + 'static,
{
    fn wrap(output: Output) -> Self {
        return Self {
            output: Box::new(output),
        };
    }
}

impl<Node> Output<Node> for BoxedOutput<Node>
    where
        Node: self::Node,
{
    const KIND: &'static str = "boxed";

    fn render(&mut self, render: <Node as RenderType<'_, Node>>::Render) -> Result<(), Error> {
        return self.output.render(render);
    }
}
