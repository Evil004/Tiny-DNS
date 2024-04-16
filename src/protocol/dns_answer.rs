use crate::parsing::Result;

use super::{dns_record::{Class, DnsRecord}, packet_buffer::PacketBuffer};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsAnswer {
    records: Vec<String>,
    response_class: Class,
    ttl: u32,
    rdata: Vec<DnsRecord>,
}

impl DnsAnswer {
    pub fn new(records: Vec<String>, response_class: Class, ttl: u32, rdata: Vec<DnsRecord>) -> Self {
        DnsAnswer {
            records,
            response_class,
            ttl,
            rdata,
        }
    }
    pub fn deserialize(packet_buffer: &mut PacketBuffer, responses_count: u16) -> Result<Self> {
        let mut domain_names = Vec::new();

        for _ in 0..responses_count {
            let domain_name = packet_buffer.read_qname()?;
            domain_names.push(domain_name);
        }

        let type_id = packet_buffer.read_u16()?;
        let response_class = Class::deserialize(packet_buffer)?;
        let ttl = packet_buffer.read_u32()?;
        let rdlength = packet_buffer.read_u16()?;
        let mut rdata = Vec::new();

        for _ in 0..rdlength {
            let type_ = DnsRecord::deserialize(packet_buffer, type_id)?;
            rdata.push(type_);
        }

        return Ok(DnsAnswer {
            records: domain_names,
            response_class,
            ttl,
            rdata,
        });
    }

    pub fn serialize(&self, packet_buffer: &mut PacketBuffer) -> Result<()> {
        for name in &self.records {
            packet_buffer.write_qname(name);
        }

        if let Some(rdata) = &self.rdata.get(0) {
            packet_buffer.write_u16(rdata.get_type());
        } else {
            return Err("No rdata found".to_string().into());
        }

        packet_buffer.write_u16(self.response_class.into());
        packet_buffer.write_u32(self.ttl);

        let mut length = 0;

        for record in &self.rdata {
            length += record.get_length();
        }

        packet_buffer.write_u16(length as u16);

        for record in &self.rdata {
            record.serialize(packet_buffer)?;
        }

        return Ok(());
    }
}
