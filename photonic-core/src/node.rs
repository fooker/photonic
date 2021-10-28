use std::time::Duration;

use anyhow::Result;

use crate::scene::NodeBuilder;

pub trait Render {
    type Element;

    fn get(&self, index: usize) -> Result<Self::Element>;

    fn map<R>(self, f: &dyn Fn(Self::Element) -> R) -> MapRender<Self, R>
    where
        Self: Sized,
    {
        return MapRender {
            source: self,
            f,
        };
    }
}

pub trait NodeDecl {
    type Element;
    type Target: Node<Element = Self::Element> + 'static;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target>
    where
        Self::Target: std::marker::Sized;

    fn map<R, F>(self, f: F) -> MapNodeDecl<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Element) -> R,
    {
        return MapNodeDecl {
            source: self,
            f,
        };
    }
}

pub trait RenderType<'a, N: Node> {
    type Render: Render<Element = N::Element> + 'a;
}

pub trait Node: for<'a> RenderType<'a, Self> + Sized {
    const KIND: &'static str;

    type Element;

    fn update(&mut self, duration: Duration) -> Result<()>;
    fn render(&self) -> Result<<Self as RenderType<Self>>::Render>;

    fn map<R, F>(self, f: F) -> MapNode<Self, F>
    where
        Self: Sized,
        for<'a> F: Fn(Self::Element) -> R,
    {
        return MapNode::new(self, f);
    }
}

pub struct MapRender<'a, S, R>
where
    S: Render,
{
    source: S,
    f: &'a dyn Fn(S::Element) -> R,
}

impl<'a, S, R> MapRender<'a, S, R>
where
    S: Render,
{
    pub fn new(source: S, f: &'a dyn Fn(S::Element) -> R) -> Self {
        return Self {
            source,
            f,
        };
    }
}

impl<'a, S, R> Render for MapRender<'a, S, R>
where
    S: Render,
{
    type Element = R;

    fn get(&self, index: usize) -> Result<Self::Element> {
        return self.source.get(index).map(self.f);
    }
}

pub struct MapNode<S, F> {
    source: S,
    f: F,
}

impl<S, F> MapNode<S, F> {
    pub fn new(source: S, f: F) -> Self {
        return Self {
            source,
            f,
        };
    }
}

impl<'a, S, F, R> RenderType<'a, Self> for MapNode<S, F>
where
    S: Node,
    S::Element: 'static,
    R: 'static,
    F: (Fn(S::Element) -> R) + 'static,
{
    type Render = MapRender<'a, <S as RenderType<'a, S>>::Render, R>;
}

impl<S, F, R> Node for MapNode<S, F>
where
    S: Node,
    S::Element: 'static,
    R: 'static,
    F: (Fn(S::Element) -> R) + 'static,
{
    type Element = R;

    const KIND: &'static str = S::KIND;

    fn update(&mut self, duration: Duration) -> Result<()> {
        self.source.update(duration)?;
        return Ok(());
    }

    fn render(&self) -> Result<<Self as RenderType<Self>>::Render> {
        return Ok(self.source.render()?.map(&self.f));
    }
}

pub struct MapNodeDecl<S, F> {
    source: S,
    f: F,
}

impl<S, F> MapNodeDecl<S, F> {
    pub fn new(source: S, f: F) -> Self {
        return Self {
            source,
            f,
        };
    }
}

impl<S, F, R> NodeDecl for MapNodeDecl<S, F>
where
    S: NodeDecl,
    S::Element: 'static,
    R: 'static,
    F: (Fn(S::Element) -> R) + 'static,
{
    type Element = R;
    type Target = MapNode<S::Target, F>;

    fn materialize(self, size: usize, builder: &mut NodeBuilder) -> Result<Self::Target>
    where
        Self::Target: Sized,
    {
        return Ok(self.source.materialize(size, builder)?.map(self.f));
    }
}
