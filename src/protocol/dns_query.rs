use bitvec::{order::Msb0, vec::BitVec};
use nom::IResult;

use crate::parsing::{deserialize::{BitInput, Deserialize}, serialize::Serialize};

use super::{dns_header::DnsHeader, dns_question::DnsQuestion};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsQueryPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    
}

impl Deserialize for DnsQueryPacket{
    fn deserialize(input: BitInput) -> IResult<BitInput, DnsQueryPacket> {
        let (input, header) = DnsHeader::deserialize(input)?;


        let (input, question) = DnsQuestion::deserialize(input, header.question_count)?;

        let query = DnsQueryPacket { header, question };
        return Ok((input, query));
    }
}

impl Serialize for DnsQueryPacket {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<_, _> = BitVec::new();
        vec.append(&mut self.header.serialize());
        vec.append(&mut self.question.serialize());

        return vec;
    }
}