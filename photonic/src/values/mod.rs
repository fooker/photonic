pub mod float;
pub mod int;

pub use self::float::Value as FloatValue;
pub use self::float::ValueDecl as FloatValueDecl;
pub use self::float::ValueFactory as FloatValueFactory;

pub use self::int::Value as IntValue;
pub use self::int::ValueDecl as IntValueDecl;
pub use self::int::ValueFactory as IntValueFactory;
