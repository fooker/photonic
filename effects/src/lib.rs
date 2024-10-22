#![feature(specialization)]
#![feature(never_type)]

pub mod attrs;
pub mod nodes;

pub mod easing;

#[cfg(feature = "dynamic")]
pub mod dynamic {
    use serde::de::DeserializeOwned;

    use photonic::attr::Bounded;
    use photonic::{input, AttrValue};
    use photonic_dynamic::factory::{factory, BoundAttrFactory, FreeAttrFactory, NodeFactory};
    use photonic_dynamic::registry;

    pub struct Registry;

    impl registry::Registry for Registry {
        fn node<Reg: registry::Registry>(kind: &str) -> Option<NodeFactory<Reg>> {
            return Some(match kind {
                "alert" => factory::<crate::nodes::alert::dynamic::Config>(),
                "blackout" => factory::<crate::nodes::blackout::dynamic::Config>(),
                "brightness" => factory::<crate::nodes::brightness::dynamic::Config>(),
                "color-wheel" => factory::<crate::nodes::color_wheel::dynamic::Config>(),
                "larson" => factory::<crate::nodes::larson::dynamic::Config>(),
                "noise" => factory::<crate::nodes::noise::dynamic::Config>(),
                "overlay" => factory::<crate::nodes::overlay::dynamic::Config>(),
                "raindrops" => factory::<crate::nodes::raindrops::dynamic::Config>(),
                "select" => factory::<crate::nodes::select::dynamic::Config>(),
                "solid" => factory::<crate::nodes::solid::dynamic::Config>(),
                "splice" => factory::<crate::nodes::splice::dynamic::Config>(),
                _ => return None,
            });
        }

        fn free_attr<Reg: registry::Registry, V>(kind: &str) -> Option<FreeAttrFactory<Reg, V>>
        where V: AttrValue + DeserializeOwned + input::Coerced {
            return Some(match kind {
                "button" => factory::<crate::attrs::button::dynamic::Config<V>>(),
                "switch" => factory::<crate::attrs::switch::dynamic::Config<V>>(),
                "fader" => factory::<crate::attrs::fader::dynamic::Config<V>>(),
                "sequence" => factory::<crate::attrs::sequence::dynamic::Config<V>>(),
                "peak" => factory::<crate::attrs::peak::dynamic::Config<V>>(),
                _ => return None,
            });
        }

        fn bound_attr<Reg: registry::Registry, V>(kind: &str) -> Option<BoundAttrFactory<Reg, V>>
        where V: AttrValue + DeserializeOwned + input::Coerced + Bounded {
            return Some(match kind {
                "button" => factory::<crate::attrs::button::dynamic::Config<V>>(),
                "switch" => factory::<crate::attrs::switch::dynamic::Config<V>>(),
                "fader" => factory::<crate::attrs::fader::dynamic::Config<V>>(),
                "looper" => factory::<crate::attrs::looper::dynamic::Config<V>>(),
                "noise" => factory::<crate::attrs::noise::dynamic::Config>(),
                "random" => factory::<crate::attrs::random::dynamic::Config>(),
                "sequence" => factory::<crate::attrs::sequence::dynamic::Config<V>>(),
                "peak" => factory::<crate::attrs::peak::dynamic::Config<V>>(),
                _ => return None,
            });
        }
    }
}
