use std::net::{Ipv4Addr, Ipv6Addr};

use super::Result;

use super::packet_buffer::PacketBuffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsRecordType {
    A {
        address: Ipv4Addr,
    },
    NS {
        name_server: String,
    },
    CNAME {
        canonical_name: String,
    },
    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    MX {
        priority: u16,
        exchange: String,
    },
    TXT {
        text: String,
    },
    AAAA {
        address: Ipv6Addr,
    },
}

impl DnsRecordType {
    pub fn deserialize(
        packet_buffer: &mut PacketBuffer,
        type_id: u16,
        rdlength: u16,
    ) -> Result<Self> {
        match type_id {
            1 => {
                let address = Ipv4Addr::new(
                    packet_buffer.read()?,
                    packet_buffer.read()?,
                    packet_buffer.read()?,
                    packet_buffer.read()?,
                );
                Ok(DnsRecordType::A { address })
            }
            2 => {
                let name_server = packet_buffer.read_qname()?;
                Ok(DnsRecordType::NS { name_server })
            }
            5 => {
                let canonical_name = packet_buffer.read_qname()?;
                Ok(DnsRecordType::CNAME { canonical_name })
            }
            6 => {
                let mname = packet_buffer.read_qname()?;
                let rname = packet_buffer.read_qname()?;
                let serial = packet_buffer.read_u32()?;
                let refresh = packet_buffer.read_u32()?;
                let retry = packet_buffer.read_u32()?;
                let expire = packet_buffer.read_u32()?;
                let minimum = packet_buffer.read_u32()?;

                return Ok(Self::SOA {
                    mname,
                    rname,
                    serial,
                    refresh,
                    retry,
                    expire,
                    minimum,
                });
            }
            15 => {
                let priority = packet_buffer.read_u16()?;
                let exchange = packet_buffer.read_qname()?;
                Ok(DnsRecordType::MX { priority, exchange })
            }
            16 => {
                let text = packet_buffer.read_bytes(rdlength as usize)?;
                Ok(DnsRecordType::TXT {
                    text: String::from_utf8(text)?,
                })
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
                Ok(DnsRecordType::AAAA { address })
            }
            _ => Err("Unknown type".to_string().into()),
        }
    }

    pub fn get_type(&self) -> u16 {
        match self {
            DnsRecordType::A { .. } => 1,
            DnsRecordType::NS { .. } => 2,
            DnsRecordType::CNAME { .. } => 5,
            DnsRecordType::SOA { .. } => 6,
            DnsRecordType::MX { .. } => 15,
            DnsRecordType::TXT { .. } => 16,
            DnsRecordType::AAAA { .. } => 28,
        }
    }

    pub(crate) fn serialize(&self, packet_buffer: &mut PacketBuffer) -> Result<()> {

        let start_pos = packet_buffer.pos;
        packet_buffer.write_u16(0); // Placeholder for length

        match self {
            DnsRecordType::A { address } => {
                packet_buffer.write(address.octets()[0]);
                packet_buffer.write(address.octets()[1]);
                packet_buffer.write(address.octets()[2]);
                packet_buffer.write(address.octets()[3]);
            }
            DnsRecordType::NS { name_server } => {
                packet_buffer.write_qname(name_server);
            }
            DnsRecordType::CNAME { canonical_name } => {
                packet_buffer.write_qname(canonical_name);
            }
            DnsRecordType::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => {
                packet_buffer.write_qname(mname);
                packet_buffer.write_qname(rname);
                packet_buffer.write_u32(*serial);
                packet_buffer.write_u32(*refresh);
                packet_buffer.write_u32(*retry);
                packet_buffer.write_u32(*expire);
                packet_buffer.write_u32(*minimum);
            }
            DnsRecordType::MX { priority, exchange } => {
                packet_buffer.write_u16(*priority);
                packet_buffer.write_qname(exchange);
            }
            DnsRecordType::TXT { text } => {
                packet_buffer.write_bytes(text.as_bytes().to_vec());
            }
            DnsRecordType::AAAA { address } => {
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

        let end_pos = packet_buffer.pos;
        let length = end_pos - start_pos - 2; // <--- A piti le da toc :D 

        packet_buffer.seek(start_pos);
        packet_buffer.write_u16(length as u16);
        packet_buffer.seek(end_pos);
        
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

impl Class {
    pub fn from_u16(value: u16) -> Result<Self> {
        match value {
            1 => Ok(Class::IN),
            2 => Ok(Class::CS),
            3 => Ok(Class::CH),
            4 => Ok(Class::HS),
            _ => Err("Unknown class".to_string().into()),
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
        Ok(Class::from_u16(class)?)
    }
}
