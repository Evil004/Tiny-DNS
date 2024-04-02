use nom::IResult;

use crate::parsing::BitInput;

use super::{dns_header::DnsHeader, dns_question::DnsQuestion};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsQuery {
    header: DnsHeader,
    question: DnsQuestion,
}

impl DnsQuery {
    pub fn parse_query_from_bit_input(input: BitInput) -> IResult<BitInput, DnsQuery> {
        let (input, header) = DnsHeader::deserialize_header_from_bit_input(input)?;

        let (input, question) = DnsQuestion::deserialize_from_bit_input(input)?;

        let query = DnsQuery { header, question };
        return Ok((input, query));
    }
}
