#![allow(dead_code)]

use leb128;
use nom;

use super::error::Error;

fn parse_leb128_unsigned_max(bytes: &[u8], max: usize) -> nom::IResult<&[u8], u64, Error> {
    let mut read = &bytes[.. max];
    let res = leb128::read::unsigned(&mut read);
    let consumed = max - read.len();
    match res {
        Ok(value) => nom::IResult::Done(&bytes[consumed ..], value),
        Err(_) => nom::IResult::Incomplete(nom::Needed::Unknown),
    }
}

fn parse_leb128_signed_max(bytes: &[u8], max: usize) -> nom::IResult<&[u8], i64, Error> {
    let mut read = &bytes[.. max];
    // TODO: no unwrap
    let res = leb128::read::signed(&mut read);
    let consumed = max - read.len();
    match res {
        Ok(value) => nom::IResult::Done(&bytes[consumed ..], value),
        Err(_) => nom::IResult::Incomplete(nom::Needed::Unknown),
    }
}

pub fn parse_varuint1(bytes: &[u8]) -> nom::IResult<&[u8], u64, Error> {
    parse_leb128_unsigned_max(bytes, 1)
}

pub fn parse_varuint7(bytes: &[u8]) -> nom::IResult<&[u8], u64, Error> {
    parse_leb128_unsigned_max(bytes, 1)
}

pub fn parse_varuint32(bytes: &[u8]) -> nom::IResult<&[u8], u64, Error> {
    parse_leb128_unsigned_max(bytes, 5)
}

pub fn parse_varint7(bytes: &[u8]) -> nom::IResult<&[u8], i64, Error> {
    parse_leb128_signed_max(bytes, 1)
}

pub fn parse_varint32(bytes: &[u8]) -> nom::IResult<&[u8], i64, Error> {
    parse_leb128_signed_max(bytes, 5)
}

pub fn parse_varint64(bytes: &[u8]) -> nom::IResult<&[u8], i64, Error> {
    parse_leb128_signed_max(bytes, 10)
}
