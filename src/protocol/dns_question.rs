use nom::IResult;

use crate::parsing::{
    deserialize::{BitInput, Deserialize, DeserializeWithLength},
    serialize::{serialize_n_bits, Serialize},
};

use super::{domain_names::DomainNames, Class, Type};

#[derive(Debug)]
pub struct DnsQuestion {
    pub domain_names: DomainNames,
    pub qtype: Type,
    pub qclass: Class,
}

impl DeserializeWithLength for DnsQuestion {
    fn deserialize(input: BitInput, question_count: u16) -> IResult<BitInput, Self> {
        let (input, qname) = DomainNames::deserialize(input, question_count)?;
        let (input, qtype) = Type::deserialize(input)?;
        let (input, qclass) = Class::deserialize(input)?;

        return Ok((
            input,
            DnsQuestion {
                domain_names: qname,
                qtype,
                qclass,
            },
        ));
    }
}

impl Serialize for DnsQuestion {
    fn serialize(&self) -> bitvec::prelude::BitVec<u8, bitvec::prelude::Msb0> {
        let mut vec: bitvec::prelude::BitVec<_, _> = bitvec::prelude::BitVec::new();
        vec.append(&mut self.domain_names.serialize());
        vec.append(&mut serialize_n_bits(16, self.qtype as u64));
        vec.append(&mut serialize_n_bits(16, self.qclass as u64));
        return vec;
    }
}
