use crate::parsing::Result;

use super::{packet_buffer::PacketBuffer, Class};

#[derive(Debug)]
pub struct DnsQuery {
    pub domain_names: Vec<String>,
    pub qtype: u16,
    pub qclass: Class,
}

impl DnsQuery {
    pub fn deserialize(packet_bufffer: &mut PacketBuffer, query_count: u16) -> Result<Self> {
        let mut domain_names = Vec::new();

        for _ in 0..query_count {
            let domain_name = packet_bufffer.read_qname()?;
            domain_names.push(domain_name);
        }

        dbg!(domain_names.clone());

        let qtype = packet_bufffer.read_u16()?;
        dbg!(qtype.clone());

        let qclass = Class::deserialize(packet_bufffer)?;

        return Ok(DnsQuery {
            domain_names,
            qtype,
            qclass,
        });
    }
}

impl DnsQuery {
    fn serialize(&self) -> bitvec::prelude::BitVec<u8, bitvec::prelude::Msb0> {
        /* let mut vec: bitvec::prelude::BitVec<_, _> = bitvec::prelude::BitVec::new();
        vec.append(&mut self.domain_names.serialize());
        vec.append(&mut serialize_n_bits(16, self.qtype as u64));
        vec.append(&mut serialize_n_bits(16, self.qclass as u64));
        return vec; */
        todo!()
    }
}
