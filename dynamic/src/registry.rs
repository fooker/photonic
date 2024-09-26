use serde::de::DeserializeOwned;

use photonic::attr::Bounded;
use photonic::{input, AttrValue};

use crate::factory::{BoundAttrFactory, FreeAttrFactory, NodeFactory, OutputFactory};

#[allow(unused_variables)]
pub trait Registry {
    fn node<Reg: Registry>(kind: &str) -> Option<NodeFactory<Reg>> {
        None
    }

    fn free_attr<Reg: Registry, V>(kind: &str) -> Option<FreeAttrFactory<Reg, V>>
    where V: AttrValue + DeserializeOwned + input::Coerced {
        None
    }

    fn bound_attr<Reg: Registry, V>(kind: &str) -> Option<BoundAttrFactory<Reg, V>>
    where V: AttrValue + DeserializeOwned + input::Coerced + Bounded {
        None
    }

    fn output<Reg: Registry>(kind: &str) -> Option<OutputFactory<Reg>> {
        None
    }
}

#[macro_export]
macro_rules! combine {
    ($f:ident, $kind:expr, ()) => { None };
    ($f:ident, $kind:expr, ($r1:ty)) => {
        <$r1>::$f($kind)
    };
    ($f:ident, $kind:expr, ($r1:ty, $($r2:ty),+)) => {
        ::core::option::Option::or_else(
            combine!($f, $kind, ($r1)),
            || combine!($f, $kind, ($($r2),+)))
    };
}
