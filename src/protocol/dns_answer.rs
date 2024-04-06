use bitvec::{order::Msb0, vec::BitVec};
use nom::IResult;

use crate::{
    parsing::{deserialize::{take_16bits, take_32bits, take_vec_of_n_bytes, BitInput, DeserializeWithLength}, serialize::{serialize_byte, serialize_n_bits, Serialize}},
    resolver::resolv,
};

use super::{
    dns_question::DnsQuestion,
    domain_names::{DomainNames, DomainParts},
    DNS_HEADER_SIZE_IN_BYTES,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsAnswer {
    domain_name: DomainNames,
    response_type: u16,  // 16 bits
    response_class: u16, // 16 bits
    ttl: u32,            // 32 bits
    rdlength: u16,       // 16 bits
    rdata: Vec<u8>,
}

impl DnsAnswer {
    pub fn from_query(query: &DnsQuestion, ttl: u32) -> DnsAnswer {
        let mut domain_parts = Vec::new();
        let mut rdata = Vec::new();

        for domain in query.domain_names.get_domains() {
            let resolved_name = resolv(&domain.0, query.qtype, query.qclass);

            rdata.extend_from_slice(&resolved_name.direction);

            domain_parts.push(DomainParts::Pointer {
                pos: DNS_HEADER_SIZE_IN_BYTES + domain.1 as u16,
            });
        }

        let domain_name = DomainNames::new_from_vec_with_starting_point(domain_parts);

        DnsAnswer {
            domain_name,
            response_type: query.qtype,
            response_class: query.qclass,
            ttl,
            rdlength: rdata.len() as u16,
            rdata,
        }
    }
}

impl Serialize for DnsAnswer {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();

        vec.append(&mut self.domain_name.serialize());
        vec.append(&mut serialize_n_bits(16, self.response_type as u64));
        vec.append(&mut serialize_n_bits(16, self.response_class as u64));
        vec.append(&mut serialize_n_bits(32, self.ttl as u64));

        vec.append(&mut serialize_n_bits(16, self.rdlength as u64));

        for data in &self.rdata {
            vec.append(&mut serialize_byte(*data));
        }

        return vec;
    }
}

impl DeserializeWithLength for DnsAnswer {
    fn deserialize(input: BitInput, nom_of_domains: u16) -> IResult<BitInput, Self> {
        let (input, domain_name) = DomainNames::deserialize(input, nom_of_domains)?;

        let (input, response_type) = take_16bits(input)?;
        let (input, response_class) = take_16bits(input)?;
        let (input, ttl) = take_32bits(input)?;
        let (input, rdlength) = take_16bits(input)?;
        let (input, rdata) = take_vec_of_n_bytes(input, rdlength )?;

        Ok((
            input,
            DnsAnswer {
                domain_name,
                response_type,
                response_class,
                ttl,
                rdlength,
                rdata,
            },
        ))
    }
}