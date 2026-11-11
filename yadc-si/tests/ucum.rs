//! UCUM (de)serialization of dimensions: the `serde` feature, which enables
//! `yadc/ucum`. Needs only the `mechanical` family (and `angular` / `thermal`
//! for the angle and temperature tests). A quantity's dimension is its type's
//! const parameter: `dim_of(RADIAN)`.
#![cfg(feature = "serde")]
#![feature(generic_const_exprs, min_adt_const_params)]
#![allow(incomplete_features)]

use yadc::Dimension;
use yadc_si::{ISQ, StaticSIQuantity};

/// A static quantity's dimension: its const parameter.
#[allow(dead_code)]
fn dim_of<const D: ISQ>(_: StaticSIQuantity<D>) -> ISQ {
    D
}

#[cfg(feature = "angular")]
#[test]
fn angle_dimension_ucum_round_trip() {
    use serde::Deserialize;
    use serde::de::value::{Error, StrDeserializer};

    let read = |s: &str| yadc_si::ISQ::deserialize(StrDeserializer::<Error>::new(s)).unwrap();
    use yadc_si::angular::consts::{RADIAN, STERADIAN};
    use yadc_si::dimensionless::consts::RATIO;
    assert_eq!(read("1"), dim_of(RATIO));
    assert_eq!(read("rad"), dim_of(RADIAN));
    // Radians are an axis: a steradian is rad².
    assert_eq!(read("rad2"), dim_of(STERADIAN));
}

/// A kind that is not the plain one is a UCUM annotation: a temperature is
/// `K{Absolute}`; an interval, an ordinary amount, has none.
#[cfg(feature = "thermal")]
#[test]
fn temperature_ucum_round_trip() {
    use serde::Deserialize;
    use serde::de::value::{Error, StrDeserializer};
    use yadc_si::thermal::consts::{ABSOLUTE_ZERO, KELVIN};

    let read = |s: &str| ISQ::deserialize(StrDeserializer::<Error>::new(s)).unwrap();
    assert_eq!(read("K{Absolute}"), dim_of(ABSOLUTE_ZERO));
    assert_eq!(read("K"), dim_of(KELVIN));
    // An annotation names a kind of the system.
    assert!(ISQ::deserialize(StrDeserializer::<Error>::new("K{Point}")).is_err());
}

/// An instant is a point on the line of times: `s{Affine}`.
#[test]
fn instant_ucum_round_trip() {
    use serde::Deserialize;
    use serde::de::value::{Error, StrDeserializer};
    use yadc_si::mechanical::consts::{SECOND, UNIX_EPOCH};

    let read = |s: &str| ISQ::deserialize(StrDeserializer::<Error>::new(s)).unwrap();
    assert_eq!(read("s{Affine}"), dim_of(UNIX_EPOCH));
    assert_eq!(read("s"), dim_of(SECOND));
}

/// A level carries the exponents of what it is the level of; a gain has none.
#[test]
fn logarithm_ucum_round_trip() {
    use serde::Deserialize;
    use serde::de::value::{Error, StrDeserializer};
    use yadc_si::mechanical::consts::{BEL, WATT_LEVEL};

    let read = |s: &str| ISQ::deserialize(StrDeserializer::<Error>::new(s)).unwrap();
    assert_eq!(read("1{Log}"), dim_of(BEL));
    assert_eq!(read("m2.kg.s-3{Log}"), dim_of(WATT_LEVEL));
}

#[test]
fn dimensionless_serializes_as_one() {
    use serde::Deserialize;
    use serde::de::value::{Error, StrDeserializer};
    use serde::ser::{Impossible, Serialize, Serializer};

    // The smallest serializer that captures a string: a dimension serializes
    // as its UCUM string and nothing else.
    struct Str;
    macro_rules! reject {
        ($($method:ident($($arg:ty),*) -> $ret:ty;)*) => {$(
            fn $method(self, $(_: $arg),*) -> Result<$ret, Error> {
                Err(serde::ser::Error::custom("not a string"))
            }
        )*};
    }
    type No = Impossible<String, Error>;
    impl Serializer for Str {
        type Ok = String;
        type Error = Error;
        type SerializeSeq = No;
        type SerializeTuple = No;
        type SerializeTupleStruct = No;
        type SerializeTupleVariant = No;
        type SerializeMap = No;
        type SerializeStruct = No;
        type SerializeStructVariant = No;
        fn serialize_str(self, v: &str) -> Result<String, Error> {
            Ok(v.to_owned())
        }
        fn serialize_some<T: Serialize + ?Sized>(self, _: &T) -> Result<String, Error> {
            Err(serde::ser::Error::custom("not a string"))
        }
        fn serialize_newtype_struct<T: Serialize + ?Sized>(
            self,
            _: &'static str,
            _: &T,
        ) -> Result<String, Error> {
            Err(serde::ser::Error::custom("not a string"))
        }
        fn serialize_newtype_variant<T: Serialize + ?Sized>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> Result<String, Error> {
            Err(serde::ser::Error::custom("not a string"))
        }
        reject! {
            serialize_bool(bool) -> String;
            serialize_i8(i8) -> String;
            serialize_i16(i16) -> String;
            serialize_i32(i32) -> String;
            serialize_i64(i64) -> String;
            serialize_u8(u8) -> String;
            serialize_u16(u16) -> String;
            serialize_u32(u32) -> String;
            serialize_u64(u64) -> String;
            serialize_f32(f32) -> String;
            serialize_f64(f64) -> String;
            serialize_char(char) -> String;
            serialize_bytes(&[u8]) -> String;
            serialize_none() -> String;
            serialize_unit() -> String;
            serialize_unit_struct(&'static str) -> String;
            serialize_unit_variant(&'static str, u32, &'static str) -> String;
            serialize_seq(Option<usize>) -> No;
            serialize_tuple(usize) -> No;
            serialize_tuple_struct(&'static str, usize) -> No;
            serialize_tuple_variant(&'static str, u32, &'static str, usize) -> No;
            serialize_map(Option<usize>) -> No;
            serialize_struct(&'static str, usize) -> No;
            serialize_struct_variant(&'static str, u32, &'static str, usize) -> No;
        }
    }

    let wire = ISQ::DIMENSIONLESS.serialize(Str).unwrap();
    assert_eq!(wire, "1");
    let back = ISQ::deserialize(StrDeserializer::<Error>::new(&wire)).unwrap();
    assert_eq!(back, ISQ::DIMENSIONLESS);
}

/// A repeated symbol multiplies, as UCUM reads it: `m.m` is m², `m2.m-2` is
/// a plain number.
#[test]
fn repeated_symbols_multiply() {
    use serde::Deserialize;
    use serde::de::value::{Error, StrDeserializer};

    let read = |s: &str| ISQ::deserialize(StrDeserializer::<Error>::new(s)).unwrap();
    assert_eq!(read("m.m"), read("m2"));
    assert_eq!(read("m2.m-2"), ISQ::DIMENSIONLESS);
}
