use nom::IResult;

use crate::parsing::deserialize::{deserialize_domain_names, take_16bits, BitInput};

use super::domain_names::DomainNames;

#[derive(Debug)]
pub struct DnsQuestion {
    pub qname: DomainNames,
    pub qtype: u16,
    pub qclass: u16,
}

impl DnsQuestion {
    pub fn deserialize(input: BitInput, question_count: u16) -> IResult<BitInput, Self> {
        let (input, qname) = deserialize_domain_names(input, question_count)?;
        let (input, qtype) = take_16bits(input)?;
        let (input, qclass) = take_16bits(input)?;
        return Ok((
            input,
            DnsQuestion {
                qname,
                qtype,
                qclass,
            },
        ));
    }
}

/* impl Serialize for DnsQuestion{
    fn serialize(&self) -> bitvec::prelude::BitVec<u8, bitvec::prelude::Msb0> {
        let mut vec: bitvec::prelude::BitVec<_, _> = bitvec::prelude::BitVec::new();
        vec.append(&mut serialize_qname(self.qname.clone()));
        vec.push(false);
        vec.append(&mut serialize_16bits_to_bit_vec(self.qtype));
        vec.append(&mut serialize_16bits_to_bit_vec(self.qclass));
        return vec;
    }
} */
