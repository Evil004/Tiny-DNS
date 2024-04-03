use bitvec::{order::Msb0, vec::BitVec};

use crate::parsing::serialize::{
    serialize_16bits_to_bit_vec, serialize_32bits_to_bit_vec, serialize_byte, Serialize,
};

use super::{dns_question::DnsQuestion, domain_names::DomainNames};

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
    pub fn from_query(query: &DnsQuestion, ttl: u32, rdata: Vec<u8>) -> DnsAnswer {
        return DnsAnswer {
            domain_name: query.qname.clone(),
            response_type: query.qtype,
            response_class: query.qclass,
            ttl: ttl,
            rdlength: rdata.len() as u16,
            rdata: rdata,
        };
    }
}

impl Serialize for DnsAnswer {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();

        vec.append(&mut serialize_16bits_to_bit_vec(self.response_type));
        vec.append(&mut serialize_16bits_to_bit_vec(self.response_class));
        vec.append(&mut serialize_32bits_to_bit_vec(self.ttl));

        for data in &self.rdata {
            vec.append(&mut serialize_byte(*data));
        }

        return vec;
    }
}
