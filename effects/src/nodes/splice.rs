use anyhow::{bail, Result};

use photonic::{Buffer, BufferReader, Node, NodeBuilder, NodeDecl, NodeHandle, NodeRef, RenderContext};

pub struct Splice<N1, N2>
    where
        N1: NodeDecl,
        N2: NodeDecl,
{
    pub n1: NodeHandle<N1>,
    pub n2: NodeHandle<N2>,

    pub split: isize,
}

pub struct SpliceNode<N1, N2>
    where
        N1: Node + 'static,
        N2: Node + 'static,
{
    n1: NodeRef<N1>,
    n2: NodeRef<N2>,
}

fn calculate_split(split: isize, size: usize) -> Result<usize> {
    let size = size as isize;

    if 0 <= split && split <= size {
        return Ok(split as usize);
    }

    if -size <= split && split <= 0 {
        let split = size + split;
        return Ok(split as usize);
    }

    bail!("Splice split out of bounds: {} <= {} <= {}", -size, split, size);
}

impl<N1, N2, E> NodeDecl for Splice<N1, N2>
    where
        N1: NodeDecl,
        N2: NodeDecl,
        <N1 as NodeDecl>::Node: Node<Element=E> + 'static,
        <N2 as NodeDecl>::Node: Node<Element=E> + 'static,
        E: Default + Copy,
{
    type Node = SpliceNode<N1::Node, N2::Node>;

    async fn materialize(self, builder: &mut NodeBuilder<'_>) -> Result<Self::Node> {
        let split = calculate_split(self.split, builder.size)?;

        eprintln!("split={}, size={} > {}", self.split, builder.size, split);

        return Ok(Self::Node {
            n1: builder.node_with_size("n1", self.n1, split).await?,
            n2: builder.node_with_size("n2", self.n2, builder.size - split).await?,
        });
    }
}

impl<N1, N2, E> Node for SpliceNode<N1, N2>
    where
        N1: Node<Element=E> + 'static,
        N2: Node<Element=E> + 'static,
        E: Default + Copy,
{
    const KIND: &'static str = "splice";

    type Element = E;

    fn update(&mut self, ctx: &RenderContext, out: &mut Buffer<Self::Element>) -> Result<()> {
        let n1 = &ctx[self.n1];
        let n2 = &ctx[self.n2];

        for i in 0..out.size() {
            if i < n1.size() {
                out[i] = n1.get(i);
            } else {
                out[i] = n2.get(i - n1.size());
            }
        }

        return Ok(());
    }
}

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::Deserialize;

    use photonic_dynamic::{BoxedNodeDecl, config};
    use photonic_dynamic::factory::Producible;

    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub n1: config::Node,
        pub n2: config::Node,
        pub split: isize,
    }

    impl Producible for Splice<BoxedNodeDecl, BoxedNodeDecl> {
        type Config = Config;
    }

    pub fn node<B>(config: Config, builder: &mut B) -> Result<Splice<BoxedNodeDecl, BoxedNodeDecl>>
        where
            B: photonic_dynamic::NodeBuilder,
    {
        return Ok(Splice {
            n1: builder.node("n1", config.n1)?,
            n2: builder.node("n2", config.n2)?,
            split: config.split,
        });
    }
}
