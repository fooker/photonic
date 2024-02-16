use anyhow::Result;
use serde::de::DeserializeOwned;

use photonic::input::InputValue;
use photonic::AttrValue;

use crate::config::Anything;

// TODO: Do not limit attribute values to input value types but allow for casting
pub trait DynAttrValue: AttrValue + InputValue {
    fn parse(value: Anything) -> Result<Self>;
}

impl<T> DynAttrValue for T
where T: AttrValue + DeserializeOwned + InputValue
{
    fn parse(value: Anything) -> Result<Self> {
        return Ok(T::deserialize(value)?);
    }
}
