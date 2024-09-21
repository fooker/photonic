pub mod attrs;
pub mod nodes;

pub mod easing;

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::de::DeserializeOwned;

    use photonic::attr::Bounded;
    use photonic::AttrValue;
    use photonic_dynamic::factory::{factory, BoundAttrFactory, FreeAttrFactory, NodeFactory};
    use photonic_dynamic::{registry, NodeBuilder};

    pub struct Registry;

    impl<B> registry::Registry<B> for Registry
    where B: NodeBuilder + 'static
    {
        fn node(kind: &str) -> Option<NodeFactory<B>> {
            return Some(match kind {
                "alert" => factory(crate::nodes::alert::dynamic::node),
                "blackout" => factory(crate::nodes::blackout::dynamic::node),
                "brightness" => factory(crate::nodes::brightness::dynamic::node),
                "color-wheel" => factory(crate::nodes::color_wheel::dynamic::node),
                "larson" => factory(crate::nodes::larson::dynamic::node),
                "noise" => factory(crate::nodes::noise::dynamic::node),
                "overlay" => factory(crate::nodes::overlay::dynamic::node),
                "raindrops" => factory(crate::nodes::raindrops::dynamic::node),
                "select" => factory(crate::nodes::select::dynamic::node),
                "solid" => factory(crate::nodes::solid::dynamic::node),
                "splice" => factory(crate::nodes::splice::dynamic::node),
                _ => return None,
            });
        }

        fn free_attr<V>(kind: &str) -> Option<FreeAttrFactory<B, V>>
        where V: AttrValue + DeserializeOwned {
            return Some(match kind {
                "button" => factory(crate::attrs::button::dynamic::free_attr),
                // "fader" => factory(crate::attrs::fader::dynamic::free_attr),
                "sequence" => factory(crate::attrs::sequence::dynamic::free_attr),
                _ => return None,
            });
        }

        fn bound_attr<V>(kind: &str) -> Option<BoundAttrFactory<B, V>>
        where V: AttrValue + DeserializeOwned + Bounded {
            return Some(match kind {
                "button" => factory(crate::attrs::button::dynamic::bound_attr),
                // "fader" => factory(crate::attrs::fader::dynamic::bound_attr),
                // "looper" => factory(crate::attrs::looper::dynamic::bound_attr),
                // "random" => factory(crate::attrs::random::dynamic::bound_attr),
                "sequence" => factory(crate::attrs::sequence::dynamic::bound_attr),
                _ => return None,
            });
        }
    }
}
