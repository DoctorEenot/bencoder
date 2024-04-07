use num_traits::ops::checked::{CheckedAdd, CheckedMul};

use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};

use crate::error::Error;

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Self { input }
    }
    fn peek_byte(&self) -> Result<u8, Error> {
        self.input.get(0).map(|v| v.clone()).ok_or(Error::Eof)
    }

    fn next_byte(&mut self) -> Result<u8, Error> {
        let ch = self.peek_byte()?;
        self.input = &self.input[1..];

        Ok(ch)
    }
    fn parse_signed<T>(&mut self) -> Result<T, Error>
    where
        T: CheckedAdd + CheckedMul + From<i8>,
    {
        if self.next_byte()? != b'i' {
            return Err(Error::ExpectedInteger);
        }
        let is_positive = if self.peek_byte()? != b'-' {
            T::from(1)
        } else {
            self.input = &self.input[1..];
            T::from(-1)
        };
        let mut integer = match self.next_byte()? {
            ch @ b'0'..=b'9' => T::from(
                (ch as i8)
                    .checked_sub(b'0' as i8)
                    .ok_or(Error::LargeNumber)?,
            ),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        let mut counter: usize = 0;
        let mut input_iterator = self.input.iter();
        let mut closing_tag_found = false;
        while let Some(char) = input_iterator.next() {
            match char {
                ch @ b'0'..=b'9' => {
                    counter += 1;
                    integer = integer
                        .checked_mul(&T::from(10))
                        .ok_or(Error::LargeNumber)?;
                    integer = integer
                        .checked_add(&T::from(
                            (*ch as i8)
                                .checked_sub(b'0' as i8)
                                .ok_or(Error::LargeNumber)?,
                        ))
                        .ok_or(Error::LargeNumber)?;
                }
                b'e' => {
                    counter += 1;
                    closing_tag_found = true;
                    break;
                }
                _ => {
                    return Err(Error::ExpectedInteger);
                }
            }
        }
        if !closing_tag_found {
            return Err(Error::ClosingTagNotFound);
        }

        self.input = &self.input[counter..];
        Ok(integer * is_positive)
    }

    fn parse_unsigned<T>(&mut self) -> Result<T, Error>
    where
        T: CheckedAdd + CheckedMul + From<u8>,
    {
        if self.next_byte()? != b'i' {
            return Err(Error::ExpectedInteger);
        }
        if self.peek_byte()? == b'-' {
            return Err(Error::ExpectedUnsignedInteger);
        }
        let mut integer = match self.next_byte()? {
            ch @ b'0'..=b'9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        let mut counter: usize = 0;
        let mut input_iterator = self.input.iter();
        let mut closing_tag_found = false;
        while let Some(char) = input_iterator.next() {
            match char {
                ch @ b'0'..=b'9' => {
                    counter += 1;
                    integer = integer
                        .checked_mul(&T::from(10))
                        .ok_or(Error::LargeNumber)?;
                    integer = integer
                        .checked_add(&T::from(
                            (*ch as u8).checked_sub(b'0').ok_or(Error::LargeNumber)?,
                        ))
                        .ok_or(Error::LargeNumber)?;
                }
                b'e' => {
                    counter += 1;
                    closing_tag_found = true;
                    break;
                }
                _ => {
                    return Err(Error::ExpectedInteger);
                }
            }
        }
        if !closing_tag_found {
            return Err(Error::ClosingTagNotFound);
        }

        self.input = &self.input[counter..];
        Ok(integer)
    }

    fn parse_byte(&mut self) -> Result<u8, Error> {
        match self.next_byte()? {
            b'1' => 1usize,
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        match self.next_byte()? {
            b':' => {}
            _ => {
                return Err(Error::ClosingTagNotFound);
            }
        };
        let char = self.next_byte()?;

        Ok(char)
    }

    fn parse_byte_string(&mut self) -> Result<Vec<u8>, Error> {
        let mut size: usize = match self.next_byte()? {
            ch @ b'1'..=b'9' => usize::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        let mut counter: usize = 0;
        let mut input_iterator = self.input.iter();
        let mut closing_tag_found = false;
        while let Some(char) = input_iterator.next() {
            match char {
                ch @ b'0'..=b'9' => {
                    counter += 1;
                    size = size.checked_mul(10).ok_or(Error::LargeNumber)?;
                    size = size
                        .checked_add(
                            (*ch as u8).checked_sub(b'0').ok_or(Error::LargeNumber)? as usize
                        )
                        .ok_or(Error::LargeNumber)?;
                }
                b':' => {
                    counter += 1;
                    closing_tag_found = true;
                    break;
                }
                _ => {
                    return Err(Error::ExpectedInteger);
                }
            }
        }
        if !closing_tag_found {
            return Err(Error::ClosingTagNotFound);
        }
        self.input = &self.input[counter..];

        if self.input.len() < size {
            return Err(Error::BadStringSize);
        }
        let to_return = Vec::from_iter(self.input[..size].iter().cloned());

        self.input = &self.input[size..];

        Ok(to_return)
    }

    fn parse_byte_string_borrowed(&mut self) -> Result<&'de [u8], Error> {
        let mut size: usize = match self.next_byte()? {
            ch @ b'1'..=b'9' => usize::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };
        let mut counter: usize = 0;
        let mut input_iterator = self.input.iter();
        let mut closing_tag_found = false;
        while let Some(char) = input_iterator.next() {
            match char {
                ch @ b'0'..=b'9' => {
                    counter += 1;
                    size = size.checked_mul(10).ok_or(Error::LargeNumber)?;
                    size = size
                        .checked_add(
                            (*ch as u8).checked_sub(b'0').ok_or(Error::LargeNumber)? as usize
                        )
                        .ok_or(Error::LargeNumber)?;
                }
                b':' => {
                    counter += 1;
                    closing_tag_found = true;
                    break;
                }
                _ => {
                    return Err(Error::ExpectedInteger);
                }
            }
        }
        if !closing_tag_found {
            return Err(Error::ClosingTagNotFound);
        }
        self.input = &self.input[counter..];

        if self.input.len() < size {
            return Err(Error::BadStringSize);
        }
        let to_return = &self.input[..size];

        self.input = &self.input[size..];

        Ok(to_return)
    }
}

impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek_byte()? {
            b'i' => {
                if let Some(sign) = self.input.get(1) {
                    if sign.eq(&b'-') {
                        self.deserialize_i64(visitor)
                    } else {
                        self.deserialize_u64(visitor)
                    }
                } else {
                    Err(Error::Syntax)
                }
            }
            b'1'..=b'9' => self.deserialize_str(visitor),
            b'd' => self.deserialize_map(visitor),
            b'l' => self.deserialize_seq(visitor),
            _ => {
                todo!()
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let num = self.parse_unsigned::<u8>()?;
        if num == 0 {
            visitor.visit_bool(false)
        } else if num == 1 {
            visitor.visit_bool(true)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidValue("Cannot deserialize f32"))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::InvalidValue("Cannot deserialize f64"))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let parsed_string = self.parse_byte_string()?;
        if parsed_string.len() > 4 {
            return Err(Error::TooBigChar);
        }
        let num: u32 = match parsed_string.len() {
            1 => parsed_string[0] as u32,
            2 => (parsed_string[0] as u32) << 8 | parsed_string[1] as u32,
            3 => {
                (parsed_string[0] as u32) << 16
                    | (parsed_string[1] as u32) << 8
                    | parsed_string[2] as u32
            }
            4 => {
                (parsed_string[0] as u32) << 24
                    | (parsed_string[1] as u32) << 16
                    | (parsed_string[2] as u32) << 8
                    | parsed_string[3] as u32
            }
            _ => 0, // impossible to get this branch
        };
        visitor.visit_char(unsafe { char::from_u32_unchecked(num) })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let parsed_string = self.parse_byte_string_borrowed()?;
        visitor.visit_str(unsafe { std::str::from_utf8_unchecked(parsed_string) })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let parsed_string = self.parse_byte_string()?;
        visitor.visit_string(unsafe { String::from_utf8_unchecked(parsed_string) })
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_byte_string_borrowed()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.parse_byte_string()?)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.next_byte()? == b'l' {
            let value = visitor.visit_seq(&mut self)?;
            if self.next_byte()? == b'e' {
                Ok(value)
            } else {
                Err(Error::ClosingTagNotFound)
            }
        } else {
            Err(Error::ExpectedArray)
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.next_byte()? == b'd' {
            let value = visitor.visit_map(&mut self)?;
            if self.next_byte()? == b'e' {
                Ok(value)
            } else {
                Err(Error::ClosingTagNotFound)
            }
        } else {
            Err(Error::ExpectedDictionary)
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.peek_byte()? {
            b'1'..=b'9' => visitor.visit_enum(unsafe {
                String::from_utf8_unchecked(self.parse_byte_string()?).into_deserializer()
            }),
            b'd' => {
                self.next_byte()?;
                let value = visitor.visit_enum(Enum::new(self))?;
                if self.next_byte()? == b'e' {
                    Ok(value)
                } else {
                    Err(Error::ClosingTagNotFound)
                }
            }
            _ => Err(Error::ExpectedEnum),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de, 'a> SeqAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.peek_byte()? == b'e' {
            return Ok(None);
        }
        seed.deserialize(&mut *self).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.peek_byte()? == b'e' {
            return Ok(None);
        }
        seed.deserialize(&mut *self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;

        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Error::ExpectedString)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}

use serde::Deserialize;
pub fn from_bytes<'a, T>(b: &'a [u8]) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(b);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::*;
    use serde::{self, Deserialize};
    #[test]
    fn test() {
        #[derive(Deserialize, Debug)]
        pub struct AnnouncePeer {
            id: String,
            info_hash: String,
            port: u16,
            implied_port: bool,
        }

        let string_test = "d2:id9:picked id9:info_hash9:info hash4:porti228e12:implied_porti1ee";

        let mut deserializer = Deserializer::from_bytes(string_test.as_bytes());
        let t = AnnouncePeer::deserialize(&mut deserializer).unwrap();

        println!("{:?}", t);
    }

    #[test]
    fn test_with() {
        #[derive(Deserialize, Debug)]
        pub struct AnnouncePeer {
            #[serde(deserialize_with = "binary_string::deserialize")]
            id: Vec<u8>,
            info_hash: String,
            port: u16,
            implied_port: bool,
        }

        let string_test = String::from("d2:id6:\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}9:info_hash9:info hash4:porti228e12:implied_porti1ee");
        let mut deserializer = Deserializer::from_bytes(string_test.as_bytes());
        let t = AnnouncePeer::deserialize(&mut deserializer).unwrap();

        println!("{:?}", t);
    }
}
