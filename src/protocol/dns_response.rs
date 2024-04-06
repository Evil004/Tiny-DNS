use bitvec::{order::Msb0, vec::BitVec};
use nom::IResult;

use crate::parsing::{
    deserialize::{BitInput, Deserialize, DeserializeWithLength},
    serialize::Serialize,
};

use super::{
    dns_answer::DnsAnswer, dns_header::DnsHeader, dns_query::DnsQueryPacket,
    dns_question::DnsQuestion,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsResponsePacket {
    header: DnsHeader,
    query: DnsQuestion,
    answers: DnsAnswer,
}

impl DnsResponsePacket {
    pub fn from_query(mut query: DnsQueryPacket, ttl: u32) -> DnsResponsePacket {
        let answers = DnsAnswer::from_query(&query.question, ttl);

        query.header.set_as_response(1);

        return DnsResponsePacket {
            header: query.header,
            query: query.question,
            answers: answers,
        };
    }
}

impl Deserialize for DnsResponsePacket {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self> {
        let (input, header) = DnsHeader::deserialize(input)?;
        let (input, query) = DnsQuestion::deserialize(input, header.question_count)?;
        let (input, answers) = DnsAnswer::deserialize(input, header.answer_count)?;

        return Ok((
            input,
            DnsResponsePacket {
                header: header,
                query: query,
                answers: answers,
            },
        ));
    }
}

impl Serialize for DnsResponsePacket {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();
        vec.append(&mut self.header.serialize());
        vec.append(&mut self.query.serialize());
        vec.append(&mut self.answers.serialize());

        return vec;
    }
}
