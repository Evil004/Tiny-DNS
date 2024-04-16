use crate::parsing::{deserialize::Deserialize, Result};

use super::{dns_answer::DnsAnswer, dns_header::DnsHeader, dns_query::DnsQuery, packet_buffer::{self, PacketBuffer}};


#[derive(Debug)]
pub struct DnsPacket {
    header: DnsHeader,
    questions: DnsQuery,
    answer: Option<DnsAnswer>,
}

impl DnsPacket{
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {

        let header = DnsHeader::deserialize(packet_buffer)?;
        dbg!(&header);
        let questions = DnsQuery::deserialize(packet_buffer, header.question_count)?;
        
        let mut answer = None;
        if header.answer_count > 0 {
            answer = Some(DnsAnswer::deserialize(packet_buffer, header.answer_count)?);
        }

        return Ok(DnsPacket {
            header,
            questions,
            answer,
        });

    }

    pub fn serialize (&self)-> Result<PacketBuffer> {
        let mut  packet_buffer = PacketBuffer::new([0u8;512]);

        self.header.serialize(&mut packet_buffer)?;

        return Ok(packet_buffer);
    }
}
