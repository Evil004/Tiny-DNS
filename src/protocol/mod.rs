use std::net::{Ipv4Addr, Ipv6Addr};

use bitvec::{order::Msb0, vec::BitVec};

use crate::parsing::{
    serialize::{serialize_n_bits, Serialize},
    Result,
};

use self::packet_buffer::PacketBuffer;

pub mod dns_answer;
pub mod dns_header;
pub mod dns_packet;
pub mod dns_query;
pub mod domain_names;
pub mod packet_buffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsRecord {
    A { address: Ipv4Addr },
    NS { name_server: String },
    CNAME { canonical_name: String },
    MX { priority: u16, exchange: String },
    AAAA { address: Ipv6Addr },
}

impl DnsRecord {
    fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {
        let type_ = packet_buffer.read_u16()?;
        match type_ {
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
}

impl Serialize for DnsRecord {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();

        let as_u16: u16 = match self {
            DnsRecord::A { address: _ } => 1,
            DnsRecord::NS { name_server: _ } => 2,
            DnsRecord::CNAME { canonical_name: _ } => 5,
            DnsRecord::MX {
                priority: _,
                exchange: _,
            } => 15,
            DnsRecord::AAAA { address: _ } => 28,
        };

        vec.append(&mut serialize_n_bits(16, as_u16 as u64));
        return vec;
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

impl Serialize for Class {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();
        let as_u16: u16 = (*self).into();
        vec.append(&mut serialize_n_bits(16, as_u16 as u64));
        return vec;
    }
}
