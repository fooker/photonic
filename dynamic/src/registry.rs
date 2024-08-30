use serde::de::DeserializeOwned;

use photonic::attr::Bounded;
use photonic::input::InputValue;
use photonic::AttrValue;

use crate::factory::{BoundAttrFactory, FreeAttrFactory, NodeFactory, OutputFactory};

#[allow(unused_variables)]
pub trait Registry<B>
where B: ?Sized
{
    fn node(kind: &str) -> Option<NodeFactory<B>> {
        None
    }
    fn free_attr<V>(kind: &str) -> Option<FreeAttrFactory<B, V>>
    where V: AttrValue + DeserializeOwned + InputValue {
        None
    }
    fn bound_attr<V>(kind: &str) -> Option<BoundAttrFactory<B, V>>
    where V: AttrValue + DeserializeOwned + InputValue + Bounded {
        None
    }
    fn output(kind: &str) -> Option<OutputFactory<B>> {
        None
    }
}

// pub struct Combined<R1, R2, B>(PhantomData<(R1, R2, B)>)
//     where
//         R1: Registry<B>,
//         R2: Registry<B>,
//         B: ?Sized;
//
// impl<R1, R2, B> Registry<B> for Combined<R1, R2, B>
//     where
//         R1: Registry<B>,
//         R2: Registry<B>,
//         B: ?Sized,
// {
//     fn node(kind: &str) -> Option<NodeFactory<B>> {
//         return R1::node(kind).or_else(|| R2::node(kind));
//     }
//
//     fn free_attr<V>(kind: &str) -> Option<FreeAttrFactory<B, V>>
//         where
//             V: AttrValue + DeserializeOwned + InputValue,
//     {
//         return R1::free_attr(kind).or_else(|| R2::free_attr(kind));
//     }
//
//     fn bound_attr<V>(kind: &str) -> Option<BoundAttrFactory<B, V>>
//         where
//             V: AttrValue + DeserializeOwned + InputValue + Bounded,
//     {
//         return R1::bound_attr(kind).or_else(|| R2::bound_attr(kind));
//     }
//
//     fn output(kind: &str) -> Option<OutputFactory<B>> {
//         return R1::output(kind).or_else(|| R2::output(kind));
//     }
// }

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
