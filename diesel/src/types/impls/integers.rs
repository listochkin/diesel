extern crate byteorder;

use std::error::Error;
use std::io::Write;

use backend::{Backend, Pg};
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use super::option::UnexpectedNullError;
use types::{self, FromSql, ToSql, IsNull};

impl<DB: Backend> FromSql<types::SmallInt, DB> for i16 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let mut bytes = not_none!(bytes);
        bytes.read_i16::<BigEndian>().map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<DB: Backend> ToSql<types::SmallInt, DB> for i16 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_i16::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<DB: Backend> FromSql<types::Integer, DB> for i32 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let mut bytes = not_none!(bytes);
        bytes.read_i32::<BigEndian>().map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<DB: Backend> ToSql<types::Integer, DB> for i32 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_i32::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<DB: Backend> FromSql<types::BigInt, DB> for i64 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let mut bytes = not_none!(bytes);
        bytes.read_i64::<BigEndian>().map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<DB: Backend> ToSql<types::BigInt, DB> for i64 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_i64::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl FromSql<types::Oid, Pg> for u32 {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let mut bytes = not_none!(bytes);
        bytes.read_u32::<BigEndian>().map_err(|e| e.into())
    }
}

impl ToSql<types::Oid, Pg> for u32 {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_u32::<BigEndian>(*self)
            .map(|_| IsNull::No)
            .map_err(|e| e.into())
    }
}

#[test]
fn i16_to_sql() {
    let mut bytes = vec![];
    ToSql::<types::SmallInt, Pg>::to_sql(&1i16, &mut bytes).unwrap();
    ToSql::<types::SmallInt, Pg>::to_sql(&0i16, &mut bytes).unwrap();
    ToSql::<types::SmallInt, Pg>::to_sql(&-1i16, &mut bytes).unwrap();
    assert_eq!(bytes, vec![0, 1, 0, 0, 255, 255]);
}

#[test]
fn i32_to_sql() {
    let mut bytes = vec![];
    ToSql::<types::Integer, Pg>::to_sql(&1i32, &mut bytes).unwrap();
    ToSql::<types::Integer, Pg>::to_sql(&0i32, &mut bytes).unwrap();
    ToSql::<types::Integer, Pg>::to_sql(&-1i32, &mut bytes).unwrap();
    assert_eq!(bytes, vec![0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255]);
}

#[test]
fn i64_to_sql() {
    let mut bytes = vec![];
    ToSql::<types::BigInt, Pg>::to_sql(&1i64, &mut bytes).unwrap();
    ToSql::<types::BigInt, Pg>::to_sql(&0i64, &mut bytes).unwrap();
    ToSql::<types::BigInt, Pg>::to_sql(&-1i64, &mut bytes).unwrap();
    assert_eq!(bytes, vec![
               0, 0, 0, 0, 0, 0, 0, 1,
               0, 0, 0, 0, 0, 0, 0, 0,
               255, 255, 255, 255, 255, 255, 255, 255]);
}
