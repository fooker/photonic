#![feature(try_blocks)]
#![allow(clippy::needless_return)]

use photonic_core::element::palette::LinSrgb;

pub mod io;

type Element = LinSrgb<u8>;

#[cfg(feature = "dyn")]
pub mod registry {
    use photonic_core::boxed::BoxedNodeDecl;
    use photonic_core::element::RGBColor;
    use photonic_dyn::builder::NodeBuilder;
    use photonic_dyn::registry;
    use photonic_dyn::registry::{Factory, NodeRegistry};

    pub struct Registry;

    impl NodeRegistry for Registry {
        fn manufacture<Builder: NodeBuilder>(
            kind: &str,
        ) -> Option<Box<dyn Factory<BoxedNodeDecl<RGBColor>, Builder>>> {
            return Some(match kind {
                "exec:io" => registry::node::<Builder, crate::io::IOExecNodeDecl>(),
                _ => return None,
            });
        }
    }
}
