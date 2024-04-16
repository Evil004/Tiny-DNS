use std::net::{Ipv4Addr, Ipv6Addr};

use super::Result;

use super::packet_buffer::PacketBuffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsRecord {
    A { address: Ipv4Addr },
    NS { name_server: String },
    CNAME { canonical_name: String },
    MX { priority: u16, exchange: String },
    AAAA { address: Ipv6Addr },
}

impl DnsRecord {
    pub fn deserialize(packet_buffer: &mut PacketBuffer, type_id: u16) -> Result<Self> {
        match type_id {
            1 => {
                let address = Ipv4Addr::new(
                    packet_buffer.read()?,
                    packet_buffer.read()?,
                    packet_buffer.read()?,
                    packet_buffer.read()?,
                );
                Ok(DnsRecord::A { address })
            }
            2 => {
                let name_server = packet_buffer.read_qname()?;
                Ok(DnsRecord::NS { name_server })
            }
            5 => {
                let canonical_name = packet_buffer.read_qname()?;
                Ok(DnsRecord::CNAME { canonical_name })
            }
            15 => {
                let priority = packet_buffer.read_u16()?;
                let exchange = packet_buffer.read_qname()?;
                Ok(DnsRecord::MX { priority, exchange })
            }
            28 => {
                let address = Ipv6Addr::new(
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                    packet_buffer.read_u16()?,
                );
                Ok(DnsRecord::AAAA { address })
            }
            _ => Err("Unknown type".to_string().into()),
        }
    }

    pub fn get_type(&self) -> u16 {
        match self {
            DnsRecord::A { .. } => 1,
            DnsRecord::NS { .. } => 2,
            DnsRecord::CNAME { .. } => 5,
            DnsRecord::MX { .. } => 15,
            DnsRecord::AAAA { .. } => 28,
        }
    }

    pub(crate) fn get_length(&self) -> u16 {
        match self {
            DnsRecord::A { .. } => 4,
            DnsRecord::NS { name_server } => name_server.len() as u16,
            DnsRecord::CNAME { canonical_name } => canonical_name.len() as u16,
            DnsRecord::MX { exchange, .. } => exchange.len() as u16,
            DnsRecord::AAAA { .. } => 16,
        }
    }

    pub(crate) fn serialize(&self, packet_buffer: &mut PacketBuffer) -> Result<()> {
        match self {
            DnsRecord::A { address } => {
                packet_buffer.write(address.octets()[0]);
                packet_buffer.write(address.octets()[1]);
                packet_buffer.write(address.octets()[2]);
                packet_buffer.write(address.octets()[3]);
            }
            DnsRecord::NS { name_server } => {
                for c in name_server.chars() {
                    packet_buffer.write(c as u8);
                }
            }
            DnsRecord::CNAME { canonical_name } => {
                for c in canonical_name.chars() {
                    packet_buffer.write(c as u8);
                }
            }
            DnsRecord::MX { priority, exchange } => {
                packet_buffer.write_u16(*priority);
                for c in exchange.chars() {
                    packet_buffer.write(c as u8);
                }
            }
            DnsRecord::AAAA { address } => {
                packet_buffer.write_u16(address.segments()[0]);
                packet_buffer.write_u16(address.segments()[1]);
                packet_buffer.write_u16(address.segments()[2]);
                packet_buffer.write_u16(address.segments()[3]);
                packet_buffer.write_u16(address.segments()[4]);
                packet_buffer.write_u16(address.segments()[5]);
                packet_buffer.write_u16(address.segments()[6]);
                packet_buffer.write_u16(address.segments()[7]);
            }
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl From<u16> for Class {
    fn from(value: u16) -> Self {
        match value {
            1 => Class::IN,
            2 => Class::CS,
            3 => Class::CH,
            4 => Class::HS,
            _ => panic!("Unknown class {}", value),
        }
    }
}

impl Into<u16> for Class {
    fn into(self) -> u16 {
        match self {
            Class::IN => 1,
            Class::CS => 2,
            Class::CH => 3,
            Class::HS => 4,
        }
    }
}

impl Class {
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {
        let class = packet_buffer.read_u16()?;
        dbg!(class.clone());
        Ok(class.into())
    }
}
