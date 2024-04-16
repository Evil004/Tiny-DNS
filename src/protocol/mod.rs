use bitvec::{order::Msb0, vec::BitVec};
use nom::IResult;

use crate::parsing::{deserialize::{take_16bits, BitInput, Deserialize}, serialize::{serialize_n_bits, Serialize}};

pub mod dns_answer;
pub mod dns_header;
pub mod dns_query;
pub mod dns_question;
pub mod dns_response;
pub mod domain_names;
pub mod packet_buffer;

const DNS_HEADER_SIZE_IN_BYTES: u16 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    A,
    NS,
    CNAME,
    SOA,
    PTR,
    MX,
    TXT,
    AAAA,
}

impl Into<u16> for Type {
    fn into(self) -> u16 {
        match self {
            Type::A => 1,
            Type::NS => 2,
            Type::CNAME => 5,
            Type::SOA => 6,
            Type::PTR => 12,
            Type::MX => 15,
            Type::TXT => 16,
            Type::AAAA => 28,
        }
    }
}

impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            1 => Type::A,
            2 => Type::NS,
            5 => Type::CNAME,
            6 => Type::SOA,
            12 => Type::PTR,
            15 => Type::MX,
            16 => Type::TXT,
            28 => Type::AAAA,
            _ => panic!("Unknown type"),
        }
    }
}

impl Deserialize for Type {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self> {
        let (input, value) = take_16bits(input)?;

        Ok((input, value.into()))
    }
}

impl Serialize for Type {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();
        let as_u16: u16 = (*self).into();
        vec.append(&mut serialize_n_bits(16, as_u16 as u64));
        return vec;
       
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Class::IN,
            2 => Class::CS,
            3 => Class::CH,
            4 => Class::HS,
            _ => panic!("Unknown class"),
        }
    }
}

impl Into<u16> for Class {
    fn into(self) -> u16 {
        match self {
            Class::IN => 1,
            Class::CS => 2,
            Class::CH => 3,
            Class::HS => 4,
        }
    }
}

impl Deserialize for Class {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self> {
        let (input, value) = take_16bits(input)?;

        Ok((input, value.into()))
    }
}

impl Serialize for Class {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();
        let as_u16: u16 = (*self).into();
        vec.append(&mut serialize_n_bits(16, as_u16 as u64));
        return vec;
        
    }
}
