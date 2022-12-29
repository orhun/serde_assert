use crate::{Token, Tokens};
use alloc::{string::{String, ToString}, vec};
use core::{fmt, fmt::Display};
use serde::{de, de::{Expected, Unexpected}};
use serde::de::Error as _;

#[derive(Debug)]
pub struct Deserializer {
    tokens: vec::IntoIter<Token>,

    is_human_readable: bool,
    self_describing: bool,
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if !self.self_describing {
            return Err(Error::NotSelfDescribing);
        }
        let token = self.next_token()?;
        match token {
            Token::Bool(v) => visitor.visit_bool(v),
            Token::I8(v) => visitor.visit_i8(v),
            Token::I16(v) => visitor.visit_i16(v),
            Token::I32(v) => visitor.visit_i32(v),
            Token::I64(v) => visitor.visit_i64(v),
            #[cfg(has_i128)]
            Token::I128(v) => visitor.visit_i128(v),
            Token::U8(v) => visitor.visit_u8(v),
            Token::U16(v) => visitor.visit_u16(v),
            Token::U32(v) => visitor.visit_u32(v),
            Token::U64(v) => visitor.visit_u64(v),
            #[cfg(has_i128)]
            Token::U128(v) => visitor.visit_u128(v),
            Token::F32(v) => visitor.visit_f32(v),
            Token::F64(v) => visitor.visit_f64(v),
            Token::Char(v) => visitor.visit_char(v),
            Token::Str(v) => todo!(),
            Token::Bytes(v) => todo!(),
            Token::None => visitor.visit_none(),
            Token::Some => visitor.visit_some(self),
            Token::Unit | Token::UnitStruct { .. } => visitor.visit_unit(),
            Token::UnitVariant {
                name,
                variant_index,
                variant,
            } => todo!(),
            Token::NewtypeStruct { .. } => visitor.visit_newtype_struct(self),
            Token::NewtypeVariant { .. } => todo!(),
            Token::Seq { len } => todo!(),
            Token::Tuple { len } => todo!(),
            Token::TupleStruct { .. } => todo!(),
            Token::TupleVariant { .. } => todo!(),
            Token::Map { .. } => todo!(),
            Token::Field(v) => todo!(),
            Token::Struct { .. } => todo!(),
            Token::StructVariant { .. } => todo!(),
            _ => Err(Self::Error::invalid_type((&token).into(), &visitor)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Bool(v) = token {
            visitor.visit_bool(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I8(v) = token {
            visitor.visit_i8(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I16(v) = token {
            visitor.visit_i16(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I32(v) = token {
            visitor.visit_i32(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I64(v) = token {
            visitor.visit_i64(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::I128(v) = token {
            visitor.visit_i128(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U8(v) = token {
            visitor.visit_u8(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U16(v) = token {
            visitor.visit_u16(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U32(v) = token {
            visitor.visit_u32(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U64(v) = token {
            visitor.visit_u64(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::U128(v) = token {
            visitor.visit_u128(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::F32(v) = token {
            visitor.visit_f32(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::F64(v) = token {
            visitor.visit_f64(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Char(v) = token {
            visitor.visit_char(v)
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.next_token()? {
            Token::Some => visitor.visit_some(self),
            Token::None => visitor.visit_none(),
            token => Err(Self::Error::invalid_type((&token).into(), &visitor)),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::Unit = token {
            visitor.visit_unit()
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::UnitStruct { name: struct_name } = token {
            if name == struct_name {
                visitor.visit_unit()
            } else {
                Err(Self::Error::invalid_value((&token).into(), &visitor))
            }
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let token = self.next_token()?;
        if let Token::NewtypeStruct { name: struct_name } = token {
            if name == struct_name {
                visitor.visit_newtype_struct(self)
            } else {
                Err(Self::Error::invalid_value((&token).into(), &visitor))
            }
        } else {
            Err(Self::Error::invalid_type((&token).into(), &visitor))
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

impl Deserializer {
    pub fn builder() -> Builder {
        Builder::default()
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        loop {
            let token = self.tokens.next().ok_or(Error::EndOfTokens)?;
            if !matches!(token, Token::SkippedField(_)) {
                return Ok(token);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    tokens: Option<Tokens>,

    is_human_readable: Option<bool>,
    self_describing: Option<bool>,
}

impl Builder {
    pub fn tokens(&mut self, tokens: Tokens) -> &mut Self {
        self.tokens = Some(tokens);
        self
    }

    pub fn is_human_readable(&mut self, is_human_readable: bool) -> &mut Self {
        self.is_human_readable = Some(is_human_readable);
        self
    }

    pub fn self_describing(&mut self, self_describing: bool) -> &mut Self {
        self.self_describing = Some(self_describing);
        self
    }

    pub fn build(&mut self) -> Deserializer {
        Deserializer {
            tokens: self
                .tokens
                .clone()
                .expect("no tokens provided to `Deserializer` `Builder`")
                .0
                .into_iter(),

            is_human_readable: self.is_human_readable.unwrap_or(true),
            self_describing: self.self_describing.unwrap_or(true),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    EndOfTokens,

    NotSelfDescribing,

    Custom(String),
    InvalidType(String, String),
    InvalidValue(String, String),
    InvalidLength(usize, String),
    UnknownVariant(String, &'static [&'static str]),
    UnknownField(String, &'static [&'static str]),
    MissingField(&'static str),
    DuplicateField(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndOfTokens => f.write_str("end of tokens"),
            Self::NotSelfDescribing => f.write_str("attempted to deserialize as self-describing when deserializer is not set as self-describing"),
            Self::Custom(s) => f.write_str(s),
            Self::InvalidType(unexpected, expected) => write!(f, "invalid type: expected {}, found {}", expected, unexpected),
            Self::InvalidValue(unexpected, expected) => write!(f, "invalid value: expected {}, found {}", expected, unexpected),
            Self::InvalidLength(length, expected) => write!(f, "invalid length {}, expected {}", length, expected),
            Self::UnknownVariant(variant, expected) => write!(f, "unknown variant {}, expected one of {:?}", variant, expected),
            Self::UnknownField(field, expected) => write!(f, "unknown field {}, expected one of {:?}", field, expected),
            Self::MissingField(field) => write!(f, "missing field {}", field),
            Self::DuplicateField(field) => write!(f, "duplicate field {}", field),
        }
    }
}

impl de::StdError for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(msg.to_string())
    }

    fn invalid_type(unexpected: Unexpected, expected: &dyn Expected) -> Self {
        Self::InvalidType(unexpected.to_string(), expected.to_string())
    }

    fn invalid_value(unexpected: Unexpected, expected: &dyn Expected) -> Self {
        Self::InvalidValue(unexpected.to_string(), expected.to_string())
    }

    fn invalid_length(len: usize, expected: &dyn Expected) -> Self {
        Self::InvalidLength(len, expected.to_string())
    }

    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Self::UnknownVariant(variant.to_string(), expected)
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Self::UnknownField(field.to_string(), expected)
    }

    fn missing_field(field: &'static str) -> Self {
        Self::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        Self::DuplicateField(field)
    }
}

#[cfg(test)]
mod tests {
    use crate::Token;
    use super::Error;
    use alloc::{borrow::ToOwned, format};
    use serde::de::Error as _;

    #[test]
    fn display_error_end_of_tokens() {
        assert_eq!(format!("{}", Error::EndOfTokens), "end of tokens");
    }

    #[test]
    fn display_error_not_self_describing() {
        assert_eq!(format!("{}", Error::NotSelfDescribing), "attempted to deserialize as self-describing when deserializer is not set as self-describing");
    }

    #[test]
    fn display_error_custom() {
        assert_eq!(format!("{}", Error::Custom("foo".to_owned())), "foo");
    }

    #[test]
    fn display_error_invalid_type() {
        assert_eq!(format!("{}", Error::invalid_type((&Token::Bool(true)).into(), &"foo")), "invalid type: expected foo, found boolean `true`");
    }

    #[test]
    fn display_error_invalid_value() {
        assert_eq!(format!("{}", Error::invalid_value((&Token::Bool(true)).into(), &"foo")), "invalid value: expected foo, found boolean `true`");
    }

    #[test]
    fn display_error_invalid_length() {
        assert_eq!(format!("{}", Error::invalid_length(42, &"foo")), "invalid length 42, expected foo");
    }

    #[test]
    fn display_error_unknown_variant() {
        assert_eq!(format!("{}", Error::unknown_variant("foo", &["bar", "baz"])), "unknown variant foo, expected one of [\"bar\", \"baz\"]");
    }

    #[test]
    fn display_error_unknown_field() {
        assert_eq!(format!("{}", Error::unknown_field("foo", &["bar", "baz"])), "unknown field foo, expected one of [\"bar\", \"baz\"]");
    }

    #[test]
    fn display_error_missing_field() {
        assert_eq!(format!("{}", Error::missing_field("foo")), "missing field foo");
    }

    #[test]
    fn display_error_duplicate_field() {
        assert_eq!(format!("{}", Error::duplicate_field("foo")), "duplicate field foo");
    }
}
