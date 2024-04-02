use nom::IResult;

use crate::parsing::BitInput;

use super::dns_header::DnsHeader;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsQuery {
    header: DnsHeader,
}

impl DnsQuery {
    pub fn parse_query_from_bit_input(input: BitInput) -> IResult<BitInput, DnsQuery> {
        let (input, header) = DnsHeader::parse_header_from_bit_input(input)?;
        let query = DnsQuery { header };
        return Ok((input, query));
    }
    
}