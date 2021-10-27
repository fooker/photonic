use std::time::Duration;

use anyhow::{Error, Result};

use crate::boxed::Wrap;
use crate::element::IntoElement;
use crate::node::{Node, NodeDecl, Render, RenderType};
use crate::scene::NodeBuilder;

trait AsBoxedNodeDecl<Element> {
    fn materialize(
        self: Box<Self>,
        size: usize,
        builder: &mut NodeBuilder,
    ) -> Result<BoxedNode<Element>>;
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
    ) -> Result<BoxedNode<Element>> {
        return T::materialize(*self, size, builder).map(BoxedNode::wrap);
    }
}

pub struct BoxedNodeDecl<Element> {
    decl: Box<dyn AsBoxedNodeDecl<Element>>,
}

impl<Element, Decl> Wrap<Decl> for BoxedNodeDecl<Element>
where
    Decl: self::NodeDecl + 'static,
    Decl::Target: 'static,
    Decl::Element: IntoElement<Element>,
    Element: 'static,
{
    fn wrap(decl: Decl) -> Self {
        return Self {
            decl: Box::new(decl.map(IntoElement::into_element)),
        };
    }
}

impl<Element> NodeDecl for BoxedNodeDecl<Element>
where
    Element: 'static,
{
    type Element = Element;
    type Target = BoxedNode<Element>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target>
    where
        Self::Target: Sized,
    {
        return self.decl.materialize(size, builder);
    }
}

trait AsBoxedRender<Element> {
    fn get(&self, index: usize) -> Result<Element>;
}

impl<T, Element> AsBoxedRender<Element> for T
where
    T: Render,
    <T as Render>::Element: Into<Element>,
{
    fn get(&self, index: usize) -> Result<Element> {
        return T::get(self, index).map(Into::into).map_err(Into::into);
    }
}

pub struct BoxedRender<'a, Element> {
    render: Box<dyn AsBoxedRender<Element> + 'a>,
}

impl<'a, Element, Render> Wrap<Render> for BoxedRender<'a, Element>
where
    Render: self::Render + 'a,
    <Render as self::Render>::Element: Into<Element>,
{
    fn wrap(render: Render) -> Self {
        return Self {
            render: Box::new(render),
        };
    }
}

impl<'a, Element> Render for BoxedRender<'a, Element> {
    type Element = Element;

    fn get(&self, index: usize) -> Result<Self::Element> {
        return self.render.get(index);
    }
}

trait AsBoxedNode<Element> {
    fn update(&mut self, duration: Duration) -> Result<()>;
    fn render(&self) -> Result<BoxedRender<Element>>;
}

impl<T, Element> AsBoxedNode<Element> for T
where
    T: Node,
    T::Element: Into<Element>,
{
    fn update(&mut self, duration: Duration) -> Result<()> {
        T::update(self, duration)
    }

    fn render(&self) -> Result<BoxedRender<Element>> {
        T::render(self).map(BoxedRender::wrap)
    }
}

pub struct BoxedNode<Element> {
    node: Box<dyn AsBoxedNode<Element>>,
}

impl<Element, Node> Wrap<Node> for BoxedNode<Element>
where
    Node: self::Node + 'static,
    Node::Element: Into<Element>,
{
    fn wrap(node: Node) -> Self {
        return Self {
            node: Box::new(node),
        };
    }
}

impl<'a, Element> RenderType<'a, Self> for BoxedNode<Element>
where
    Element: 'static,
    Error: 'static,
{
    type Render = BoxedRender<'a, Element>;
}

impl<Element> Node for BoxedNode<Element>
where
    Element: 'static,
    Error: 'static,
{
    const KIND: &'static str = "boxed";

    type Element = Element;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.node.update(duration)
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        self.node.render()
    }
}
