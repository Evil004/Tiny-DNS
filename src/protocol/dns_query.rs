use nom::IResult;

use crate::parsing::deserialize::{BitInput, Deserialize};

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

