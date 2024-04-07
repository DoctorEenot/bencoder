use serde::{ser, Serialize};

use crate::error::Error;

pub struct Serializer {
    output: Vec<u8>,
}

fn digits(x: usize) -> impl Iterator<Item = u8> {
    let mut place = 10000000000000000000u64;
    std::iter::from_fn(move || {
        if place > 0 {
            let digit = (x as u64 / place) % 10;
            place /= 10;
            Some(digit as u8)
        } else {
            None
        }
    })
}

impl Serializer {
    fn push_length(&mut self, len: usize) {
        if len == 0 {
            self.output.push(b'0');
            return;
        }
        let mut digits_iter = digits(len);
        let mut prev: u8;
        loop {
            prev = digits_iter.next().unwrap();
            if prev != 0 {
                break;
            }
        }

        self.output.push(prev + 48);

        for digit in digits_iter {
            self.output.push(digit + 48);
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.serialize_u32(1)
        } else {
            self.serialize_u32(1)
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend(v.to_string().as_bytes().iter());
        self.output.push(b'e');

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidValue("Cannot serialize f32"))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::InvalidValue("Cannot serialize f64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        self.serialize_bytes(v.encode_utf8(&mut buf).as_bytes());

        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.push_length(v.len());
        self.output.push(b':');
        self.output.extend_from_slice(v);

        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.output.push(b'd');
        self.serialize_str(name)?;
        value.serialize(&mut *self)?;
        self.output.push(b'e');
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output.push(b'l');
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.output.push(b'd');
        self.serialize_str(variant);
        self.output.push(b'l');
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.output.push(b'd');
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output.push(b'd');
        variant.serialize(&mut *self)?;
        self.output.push(b'd');
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'e');
        self.output.push(b'e');
        Ok(())
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;

    Ok(serializer.output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::*;
    use serde::Serialize;

    #[test]
    fn test_struct() {
        #[derive(Serialize, Debug)]
        pub struct AnnouncePeer {
            #[serde(serialize_with = "binary_string::serialize")]
            id: Vec<u8>,
            info_hash: String,
            port: u16,
            implied_port: bool,
        }
        let arbitrary_binary_data: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let packet = AnnouncePeer {
            id: arbitrary_binary_data,
            info_hash: "info hash".into(),
            port: 228,
            implied_port: false,
        };

        let mut serializer = Serializer { output: Vec::new() };
        packet.serialize(&mut serializer).unwrap();

        println!("{:?}", unsafe {
            String::from_utf8_unchecked(serializer.output)
        });
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize, Debug)]
        pub struct AnnouncePeer {
            id: String,
            info_hash: String,
            port: u16,
            implied_port: bool,
        }

        #[derive(Serialize, Debug)]
        pub enum Query {
            Request(AnnouncePeer),
            Response(String),
        }

        let packet = Query::Request(AnnouncePeer {
            id: "picked id".into(),
            info_hash: "info hash".into(),
            port: 228,
            implied_port: false,
        });

        let mut serializer = Serializer { output: Vec::new() };
        packet.serialize(&mut serializer).unwrap();

        println!("{:?}", unsafe {
            String::from_utf8_unchecked(serializer.output)
        });
    }
}
