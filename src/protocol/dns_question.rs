use nom::IResult;

use crate::parsing::{parse_qname, take_16bits, BitInput};

#[derive(Debug)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl DnsQuestion{
    pub fn deserialize_from_bit_input(input: BitInput) -> IResult<BitInput, DnsQuestion> {
        let (input, qname) = parse_qname(input)?;
        let (input, qtype) = take_16bits(input)?;
        let (input, qclass) = take_16bits(input)?;
        return Ok((input, DnsQuestion {
            qname,
            qtype,
            qclass,
        }));
    }
}