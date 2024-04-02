use nom::IResult;

use crate::parsing::BitInput;

use super::{dns_header::DnsHeader, dns_question::DnsQuestion};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsQueryPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    
}

impl DnsQueryPacket {
    pub fn parse_query_from_bit_input(input: BitInput) -> IResult<BitInput, DnsQueryPacket> {
        let (input, header) = DnsHeader::deserialize_header_from_bit_input(input)?;

        let (input, question) = DnsQuestion::deserialize_from_bit_input(input)?;

        let query = DnsQueryPacket { header, question };
        return Ok((input, query));
    }
}
