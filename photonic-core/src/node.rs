use std::time::Duration;

use anyhow::Error;

use crate::scene::NodeBuilder;

pub trait Render {
    type Element;

    fn get(&self, index: usize) -> Self::Element;

    fn map<R, F>(self, f: &F) -> MapRender<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Element) -> R,
    {
        return MapRender { source: self, f };
    }
}

pub trait NodeDecl {
    type Element;
    type Target: Node<Element = Self::Element> + 'static;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error>
    where
        Self::Target: std::marker::Sized;

    fn map<'a, R, F>(self, f: F) -> MapNodeDecl<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Element) -> R,
    {
        return MapNodeDecl { source: self, f };
    }
}

pub trait RenderType<'a, N: Node> {
    type Render: Render<Element = N::Element> + 'a;
}

pub trait Node: for<'a> RenderType<'a, Self> + Sized {
    const KIND: &'static str;

    type Element;

    fn update(&mut self, duration: Duration);
    fn render(&mut self) -> <Self as RenderType<Self>>::Render;

    fn map<R, F>(self, f: F) -> MapNode<Self, F>
    where
        Self: Sized,
        for<'a> F: Fn(Self::Element) -> R,
    {
        return MapNode::new(self, f);
    }
}

pub struct MapRender<'a, S, F> {
    source: S,
    f: &'a F,
}

impl<'a, S, F> MapRender<'a, S, F> {
    pub fn new(source: S, f: &'a F) -> Self {
        return Self { source, f };
    }
}

impl<'a, S, F, R> Render for MapRender<'a, S, F>
where
    S: Render,
    F: (Fn(S::Element) -> R) + 'a,
{
    type Element = R;

    fn get(&self, index: usize) -> Self::Element {
        return (self.f)(self.source.get(index));
    }
}

pub struct MapNode<S, F> {
    source: S,
    f: F,
}

impl<S, F> MapNode<S, F> {
    pub fn new(source: S, f: F) -> Self {
        return Self { source, f };
    }
}

impl<'a, S, F, R> RenderType<'a, Self> for MapNode<S, F>
where
    S: Node,
    F: (Fn(S::Element) -> R) + 'static,
{
    type Render = MapRender<'a, <S as RenderType<'a, S>>::Render, F>;
}

impl<S, F, R> Node for MapNode<S, F>
where
    S: Node,
    F: (Fn(S::Element) -> R) + 'static,
{
    type Element = R;

    const KIND: &'static str = S::KIND;

    fn update(&mut self, duration: Duration) {
        self.source.update(duration);
    }

    fn render(&mut self) -> <Self as RenderType<Self>>::Render {
        return self.source.render().map(&self.f);
    }
}

pub struct MapNodeDecl<S, F> {
    source: S,
    f: F,
}

impl<S, F> MapNodeDecl<S, F> {
    pub fn new(source: S, f: F) -> Self {
        return Self { source, f };
    }
}

impl<S, F, R> NodeDecl for MapNodeDecl<S, F>
where
    S: NodeDecl,
    F: (Fn(S::Element) -> R) + 'static,
{
    type Element = R;
    type Target = MapNode<S::Target, F>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target, Error>
    where
        Self::Target: std::marker::Sized,
    {
        return Ok(self.source.materialize(size, builder)?.map(self.f));
    }
}
