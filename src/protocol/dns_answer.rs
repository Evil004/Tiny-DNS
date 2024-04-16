use crate::parsing::Result;

use super::{domain_names::DomainNames, packet_buffer::PacketBuffer, Class, DnsRecord};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsAnswer {
    records: Vec<String>,
    response_class: Class,
    ttl: u32,
    rdata: Vec<DnsRecord>,
}

impl DnsAnswer {
    pub fn deserialize(packet_bufffer: &mut PacketBuffer, responses_count: u16) -> Result<Self> {
        let mut domain_names = Vec::new();

        for _ in 0..responses_count {
            let domain_name = packet_bufffer.read_qname()?;
            domain_names.push(domain_name);
        }

        let type_id = packet_bufffer.read_u16()?;
        let response_class = Class::deserialize(packet_bufffer)?;
        let ttl = packet_bufffer.read_u32()?;
        let rdlength = packet_bufffer.read_u16()?;
        let mut rdata = Vec::new();

        for _ in 0..rdlength {
            let type_ = DnsRecord::deserialize(packet_bufffer)?;
            rdata.push(type_);
        }

        return Ok(DnsAnswer {
            records: domain_names,
            response_class,
            ttl,
            rdata,
        });
    }
}
