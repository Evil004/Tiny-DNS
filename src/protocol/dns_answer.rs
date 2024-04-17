use super::dns_header::DnsHeader;
use super::Result;

use super::{
    dns_record::{Class, DnsRecordType},
    packet_buffer::PacketBuffer,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsRecord {
    record: String,
    response_class: Class,
    ttl: u32,
    rdata: DnsRecordType,
}

impl DnsRecord {
    pub fn new(record: String, response_class: Class, ttl: u32, rdata: DnsRecordType) -> Self {
        DnsRecord {
            record,
            response_class,
            ttl,
            rdata,
        }
    }
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {
        let domain_name = packet_buffer.read_qname()?;

        let type_id = packet_buffer.read_u16()?;
        let response_class = Class::deserialize(packet_buffer)?;
        let ttl = packet_buffer.read_u32()?;
        let rdlength = packet_buffer.read_u16()?;

        let rdata: DnsRecordType =
            DnsRecordType::deserialize(packet_buffer, type_id, rdlength)?;

        return Ok(DnsRecord::new(domain_name, response_class, ttl, rdata));
    }

    pub fn serialize(&self, packet_buffer: &mut PacketBuffer) -> Result<()> {
        packet_buffer.write_qname(&self.record);

        packet_buffer.write_u16(self.rdata.get_type());

        packet_buffer.write_u16(self.response_class.into());
        packet_buffer.write_u32(self.ttl);

        let mut length = self.rdata.get_length();

        packet_buffer.write_u16(length as u16);

        self.rdata.serialize(packet_buffer)?;

        return Ok(());
    }
}
