use std::fmt::Pointer;

use bitvec::{order::Msb0, vec::BitVec};

use crate::parsing::serialize::{
    serialize_16bits_to_bit_vec, serialize_32bits_to_bit_vec, serialize_byte, Serialize,
};

use super::{dns_question::DnsQuestion, domain_names::DomainNames};

const FIRST_REGISTER_TO_POINT: u16 = 12;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsAnswer {
    domain_name: Vec<u8>,
    response_type: u16,  // 16 bits
    response_class: u16, // 16 bits
    ttl: u32,            // 32 bits
    rdlength: u16,       // 16 bits
    rdata: Vec<u8>,
}

impl DnsAnswer {
    pub fn from_query(query: &DnsQuestion, ttl: u32, rdata: Vec<u8>) -> DnsAnswer {
        let pointer1 = 0b1100_0000u8;
        let pointer2 = 0b0000_1100u8;

        return DnsAnswer {
            domain_name: vec![pointer1, pointer2],
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

        vec.append(&mut serialize_byte(self.domain_name[0]));
        vec.append(&mut serialize_byte(self.domain_name[1]));
        vec.append(&mut serialize_16bits_to_bit_vec(self.response_type));
        vec.append(&mut serialize_16bits_to_bit_vec(self.response_class));
        vec.append(&mut serialize_32bits_to_bit_vec(self.ttl));

        vec.append(&mut serialize_16bits_to_bit_vec(4u16));

        for data in &self.rdata {
            vec.append(&mut serialize_byte(*data));
        }

        return vec;
    }
}
