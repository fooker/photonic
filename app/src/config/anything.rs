pub use serde_json::Value as Anything;

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum Number {
//     UnsignedInteger(u64),
//     SignedInteger(i64),
//     Decimal(f64),
// }
//
// #[derive(Debug, Clone)]
// pub enum Anything {
//     /// Represents a null value.
//     Null,
//
//     /// Represents a boolean.
//     Bool(bool),
//
//     /// Represents a number.
//     Number(Number),
//
//     /// Represents a string.
//     String(String),
//
//     /// Represents a array.
//     Array(Vec<Anything>),
//
//     /// Represents a object.
//     Object(BTreeMap<String, Anything>),
// }
//
// impl<'de> Deserialize<'de> for Anything {
//     #[inline]
//     fn deserialize<D>(deserializer: D) -> Result<Anything, D::Error>
//         where D: serde::Deserializer<'de>,
//     {
//         struct AnythingVisitor;
//
//         impl<'de> Visitor<'de> for AnythingVisitor {
//             type Value = Anything;
//
//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("any valid value")
//             }
//
//             #[inline]
//             fn visit_bool<E>(self, value: bool) -> Result<Anything, E> {
//                 return Ok(Anything::Bool(value));
//             }
//
//             #[inline]
//             fn visit_i64<E>(self, value: i64) -> Result<Anything, E> {
//                 return Ok(Anything::Number(Number::SignedInteger(value.into())));
//             }
//
//             #[inline]
//             fn visit_u64<E>(self, value: u64) -> Result<Anything, E> {
//                 return Ok(Anything::Number(Number::UnsignedInteger(value.into())));
//             }
//
//             #[inline]
//             fn visit_f64<E>(self, value: f64) -> Result<Anything, E> {
//                 return Ok(Anything::Number(Number::Decimal(value)));
//             }
//
//             #[inline]
//             fn visit_str<E>(self, value: &str) -> Result<Anything, E>
//                 where E: serde::de::Error,
//             {
//                 return self.visit_string(String::from(value));
//             }
//
//             #[inline]
//             fn visit_string<E>(self, value: String) -> Result<Anything, E> {
//                 return Ok(Anything::String(value));
//             }
//
//             #[inline]
//             fn visit_none<E>(self) -> Result<Anything, E> {
//                 return Ok(Anything::Null);
//             }
//
//             #[inline]
//             fn visit_some<D>(self, deserializer: D) -> Result<Anything, D::Error>
//                 where D: serde::Deserializer<'de>,
//             {
//                 return Deserialize::deserialize(deserializer);
//             }
//
//             #[inline]
//             fn visit_unit<E>(self) -> Result<Anything, E> {
//                 return Ok(Anything::Null);
//             }
//
//             #[inline]
//             fn visit_seq<V>(self, mut visitor: V) -> Result<Anything, V::Error>
//                 where V: SeqAccess<'de>,
//             {
//                 let mut vec = Vec::new();
//
//                 while let Some(elem) = visitor.next_element()? {
//                     vec.push(elem);
//                 }
//
//                 return Ok(Anything::Array(vec));
//             }
//
//             fn visit_map<V>(self, mut visitor: V) -> Result<Anything, V::Error>
//                 where
//                     V: MapAccess<'de>,
//             {
//                 let mut values = BTreeMap::new();
//
//                 while let Some((key, value)) = visitor.next_entry()? {
//                     values.insert(key, value);
//                 }
//
//                 return Ok(Anything::Object(values));
//             }
//         }
//
//         deserializer.deserialize_any(AnythingVisitor)
//     }
// }
//
// impl<'de> serde::Deserializer<'de> for Anything {
//     type Error = Error;
//
//     #[inline]
//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
//         where V: Visitor<'de>,
//     {
//         return match self {
//             Self::Null => visitor.visit_unit(),
//             Self::Bool(v) => visitor.visit_bool(v),
//             Self::Number(n) => n.deserialize_any(visitor),
//             Self::String(v) => visitor.visit_string(v),
//             Self::Array(v) => visit_array(v, visitor),
//             Self::Object(v) => visit_object(v, visitor),
//         };
//     }
//
//     fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
//         where V: Visitor<'de>,
//     {
//         return match self {
//             Self::Bool(v) => visitor.visit_bool(v),
//             _ => Err(self.invalid_type(&visitor)),
//         };
//     }
//
//     fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         let mut de = crate::Deserializer::from_str(&self.key);
//
//         match tri!(de.peek()) {
//             Some(b'0'..=b'9' | b'-') => {}
//             _ => return Err(Error::syntax(ErrorCode::ExpectedNumericKey, 0, 0)),
//         }
//
//         let number = tri!(de.$using(visitor));
//
//         if tri!(de.peek()).is_some() {
//             return Err(Error::syntax(ErrorCode::ExpectedNumericKey, 0, 0));
//         }
//
//         Ok(number)
//     }
//
//     fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
//         todo!()
//     }
//
//     fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         self.deserialize_string(visitor)
//     }
//
//     fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         self.deserialize_string(visitor)
//     }
//
//     fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             #[cfg(any(feature = "std", feature = "alloc"))]
//             Value::String(v) => visitor.visit_string(v),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//
//     fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         self.deserialize_byte_buf(visitor)
//     }
//
//     fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             #[cfg(any(feature = "std", feature = "alloc"))]
//             Value::String(v) => visitor.visit_string(v),
//             Value::Array(v) => visit_array(v, visitor),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//
//     #[inline]
//     fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             Value::Null => visitor.visit_none(),
//             _ => visitor.visit_some(self),
//         }
//     }
//
//     fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             Value::Null => visitor.visit_unit(),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//
//     fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         self.deserialize_unit(visitor)
//     }
//
//     #[inline]
//     fn deserialize_newtype_struct<V>(
//         self,
//         name: &'static str,
//         visitor: V,
//     ) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         #[cfg(feature = "raw_value")]
//         {
//             if name == crate::raw::TOKEN {
//                 return visitor.visit_map(crate::raw::OwnedRawDeserializer {
//                     raw_value: Some(self.to_string()),
//                 });
//             }
//         }
//
//         let _ = name;
//         visitor.visit_newtype_struct(self)
//     }
//
//     fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             Value::Array(v) => visit_array(v, visitor),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//
//     fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         self.deserialize_seq(visitor)
//     }
//
//     fn deserialize_tuple_struct<V>(
//         self,
//         _name: &'static str,
//         _len: usize,
//         visitor: V,
//     ) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         self.deserialize_seq(visitor)
//     }
//
//     fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             Value::Object(v) => visit_object(v, visitor),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//
//     fn deserialize_struct<V>(
//         self,
//         _name: &'static str,
//         _fields: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         match self {
//             Value::Array(v) => visit_array(v, visitor),
//             Value::Object(v) => visit_object(v, visitor),
//             _ => Err(self.invalid_type(&visitor)),
//         }
//     }
//
//     #[inline]
//     fn deserialize_enum<V>(
//         self,
//         _name: &str,
//         _variants: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value, Error>
//         where
//             V: Visitor<'de>,
//     {
//         let (variant, value) = match self {
//             Value::Object(value) => {
//                 let mut iter = value.into_iter();
//                 let (variant, value) = match iter.next() {
//                     Some(v) => v,
//                     None => {
//                         return Err(serde::de::Error::invalid_value(
//                             Unexpected::Map,
//                             &"map with a single key",
//                         ));
//                     }
//                 };
//                 // enums are encoded in json as maps with a single key:value pair
//                 if iter.next().is_some() {
//                     return Err(serde::de::Error::invalid_value(
//                         Unexpected::Map,
//                         &"map with a single key",
//                     ));
//                 }
//                 (variant, Some(value))
//             }
//             Value::String(variant) => (variant, None),
//             other => {
//                 return Err(serde::de::Error::invalid_type(
//                     other.unexpected(),
//                     &"string or map",
//                 ));
//             }
//         };
//
//         visitor.visit_enum(EnumDeserializer { variant, value })
//     }
//
//     fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
//         where V: Visitor<'de>,
//     {
//         return self.deserialize_string(visitor);
//     }
//
//     fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
//         where V: Visitor<'de>,
//     {
//         drop(self);
//         return visitor.visit_unit();
//     }
// }
