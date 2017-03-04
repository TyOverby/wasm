#![allow(dead_code)]

use nom;
use byteorder;
use super::error::Error as CustomError;

pub fn parse_u8(bytes: &[u8]) -> nom::IResult<&[u8], u8, CustomError> {
    if bytes.len() >= 1 {
        return nom::IResult::Done(&bytes[1..], bytes[0])
    } else {
        nom::IResult::Incomplete(nom::Needed::Size(1))
    }
}

pub fn parse_u16(bytes: &[u8]) -> nom::IResult<&[u8], u16, CustomError> {
    use byteorder::ByteOrder;
    if bytes.len() >= 2 {
        let res = byteorder::LittleEndian::read_u16(bytes);
        return nom::IResult::Done(&bytes[2..], res)
    } else {
        nom::IResult::Incomplete(nom::Needed::Size(2 - bytes.len()))
    }
}

pub fn parse_u32(bytes: &[u8]) -> nom::IResult<&[u8], u32, CustomError> {
    use byteorder::ByteOrder;
    if bytes.len() >= 4 {
        let res = byteorder::LittleEndian::read_u32(bytes);
        return nom::IResult::Done(&bytes[4..], res)
    } else {
        nom::IResult::Incomplete(nom::Needed::Size(4 - bytes.len()))
    }
}
