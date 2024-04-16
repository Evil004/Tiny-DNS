use crate::parsing::{deserialize::Deserialize, Result};

use super::{dns_answer::DnsAnswer, dns_header::DnsHeader, dns_query::DnsQuery, packet_buffer::PacketBuffer};


#[derive(Debug)]
pub struct DnsPacket {
    header: DnsHeader,
    questions: DnsQuery,
    answer: Option<DnsAnswer>,
}

impl Deserialize<Self> for DnsPacket{
    fn deserialize(packet_bufffer: &mut PacketBuffer) -> Result<Self> {

        let header = DnsHeader::deserialize(packet_bufffer)?;
        dbg!(&header);
        let questions = DnsQuery::deserialize(packet_bufffer, header.question_count)?;
        
        let mut answer = None;
        if header.answer_count > 0 {
            answer = Some(DnsAnswer::deserialize(packet_bufffer, header.answer_count)?);
        }

        return Ok(DnsPacket {
            header,
            questions,
            answer,
        });

    }
}