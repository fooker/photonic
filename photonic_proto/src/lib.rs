extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use self::decode::Error as DecodeError;
pub use self::encode::Error as EncodeError;
use self::rmps::{decode, encode};

#[derive(Clone,Debug,Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "set_fader")]
    ChangeFader {
        name: String,
        value: f64,
    },

    #[serde(rename = "trigger_button")]
    TriggerButton {
        name: String,
    },
}

#[derive(Clone,Debug,Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Update {
    #[serde(rename = "value")]
    Value {
        name: String,
        value: f64,
    }
}

impl Command {
    pub fn decode(b: &[u8]) -> Result<Self, DecodeError> {
        return decode::from_slice(b);
    }

    pub fn encode(v: &Self) -> Result<Vec<u8>, EncodeError> {
        return encode::to_vec(v);
    }
}

impl Update {
    pub fn decode(b: &[u8]) -> Result<Self, DecodeError> {
        return decode::from_slice(b);
    }

    pub fn encode(v: &Self) -> Result<Vec<u8>, EncodeError> {
        return encode::to_vec(v);
    }
}
