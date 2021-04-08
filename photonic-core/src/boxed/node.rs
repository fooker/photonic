use std::time::Duration;

use anyhow::Error;

use crate::node::{Node, NodeDecl, Render, RenderType};
use crate::scene::NodeBuilder;

trait AsBoxedNodeDecl<Element> {
    fn materialize(
        self: Box<Self>,
        size: usize,
        builder: &mut NodeBuilder,
    ) -> Result<BoxedNode<Element>, Error>;
}

impl<T, Element> AsBoxedNodeDecl<Element> for T
where
    T: NodeDecl<Element = Element>,
    T::Target: 'static,
{
    fn materialize(
        self: Box<Self>,
        size: usize,
        builder: &mut NodeBuilder,
    ) -> Result<BoxedNode<Element>, Error> {
        return T::materialize(*self, size, builder).map(BoxedNode::wrap);
    }
}

pub struct BoxedNodeDecl<Element> {
    decl: Box<dyn AsBoxedNodeDecl<Element>>,
}

impl<Element> BoxedNodeDecl<Element> {
    pub fn wrap<Decl>(decl: Decl) -> Self
    where
        Decl: NodeDecl<Element = Element> + 'static,
    {
        return Self {
            decl: Box::new(decl),
        };
    }
}

impl<Element> NodeDecl for BoxedNodeDecl<Element>
where
    Element: 'static,
{
    type Element = Element;
    type Target = BoxedNode<Element>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error>
    where
        Self::Target: Sized,
    {
        return self.decl.materialize(size, builder);
    }
}

trait AsBoxedRender<Element> {
    fn get(&self, index: usize) -> Element;
}

impl<T, Element> AsBoxedRender<Element> for T
where
    T: Render,
    <T as Render>::Element: Into<Element>,
{
    fn get(&self, index: usize) -> Element {
        return T::get(self, index).into();
    }
}

pub struct BoxedRender<'a, Element> {
    render: Box<dyn AsBoxedRender<Element> + 'a>,
}

impl<'a, Element> BoxedRender<'a, Element> {
    pub fn wrap<Render>(render: Render) -> Self
    where
        Render: self::Render + 'a,
        Render::Element: Into<Element>,
    {
        return Self {
            render: Box::new(render),
        };
    }
}

impl<'a, Element> Render for BoxedRender<'a, Element> {
    type Element = Element;

    fn get(&self, index: usize) -> Self::Element {
        return self.render.get(index);
    }
}

trait AsBoxedNode<Element> {
    fn update(&mut self, duration: Duration);
    fn render(&mut self) -> BoxedRender<Element>;
}

impl<T, Element> AsBoxedNode<Element> for T
where
    T: Node,
    T::Element: Into<Element>,
{
    fn update(&mut self, duration: Duration) {
        T::update(self, duration);
    }

    fn render(&mut self) -> BoxedRender<Element> {
        let render = T::render(self);
        return BoxedRender::wrap(render);
    }
}

pub struct BoxedNode<Element> {
    node: Box<dyn AsBoxedNode<Element>>,
}

impl<Element> BoxedNode<Element> {
    pub fn wrap<Node>(node: Node) -> Self
    where
        Node: self::Node + 'static,
        Node::Element: Into<Element>,
    {
        return Self {
            node: Box::new(node),
        };
    }
}

impl<'a, Element> RenderType<'a, Self> for BoxedNode<Element>
where
    Element: 'static,
{
    type Render = BoxedRender<'a, Element>;
}

impl<Element> Node for BoxedNode<Element>
where
    Element: 'static,
{
    type Element = Element;

    const KIND: &'static str = "boxed";

    fn update(&mut self, duration: Duration) {
        self.node.update(duration);
    }

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return self.node.render();
    }
}
