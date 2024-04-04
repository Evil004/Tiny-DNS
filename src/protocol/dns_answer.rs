use bitvec::{order::Msb0, vec::BitVec};

use crate::{
    parsing::serialize::{
        serialize_16bits_to_bit_vec, serialize_32bits_to_bit_vec, serialize_byte,
        serialize_domain_names, Serialize,
    },
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
        let query_size = query.domain_names.get_total_len();

        let mut domain_parts = Vec::new();
        let mut rdata = Vec::new();

        for domain in query.domain_names.get_domains() {
            let resolved_name = resolv(domain.0.as_ref(), query.qtype, query.qclass);

            rdata.append(resolved_name.direction.to_vec().as_mut());

            domain_parts.push(DomainParts::Pointer {
                pos: DNS_HEADER_SIZE_IN_BYTES + domain.1 as u16,
            });
        }

        let domain_names = DomainNames::new_from_vec_with_starting_point(
            domain_parts,
            query_size as u16 + DNS_HEADER_SIZE_IN_BYTES,
        );

        return DnsAnswer {
            domain_name: domain_names,
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

        // TODO: Implement domain name serialization
        vec.append(&mut serialize_domain_names(self.domain_name.clone()));
        vec.append(&mut serialize_16bits_to_bit_vec(self.response_type));
        vec.append(&mut serialize_16bits_to_bit_vec(self.response_class));
        vec.append(&mut serialize_32bits_to_bit_vec(self.ttl));

        // TODO: Implement rdlength serialization
        vec.append(&mut serialize_16bits_to_bit_vec(4u16));

        // TODO: Implement rdata serialization
        for data in &self.rdata {
            vec.append(&mut serialize_byte(*data));
        }

        return vec;
    }
}
