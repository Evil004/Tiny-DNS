use super::Result;

use super::{
    dns_answer::DnsAnswer, dns_header::DnsHeader, dns_query::DnsQuery, dns_record::DnsRecord, packet_buffer::PacketBuffer
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsPacket {
    header: DnsHeader,
    questions: DnsQuery,
    answers: Vec<DnsAnswer>,
}

impl DnsPacket {
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {
        let header = DnsHeader::deserialize(packet_buffer)?;
        let questions = DnsQuery::deserialize(packet_buffer, header.question_count)?;

        let mut answers = Vec::new();
       for _ in 0..header.answer_count {
           let answer = DnsAnswer::deserialize(packet_buffer)?;
           answers.push(answer);
           
       }

        return Ok(DnsPacket {
            header,
            questions,
            answers,
        });
    }

    pub fn serialize(&self) -> Result<PacketBuffer> {
        let mut packet_buffer = PacketBuffer::new([0u8; 512]);

        self.header.serialize(&mut packet_buffer)?;
        self.questions.serialize(&mut packet_buffer)?;
        
        for answer in self.answers.iter() {
            answer.serialize(&mut packet_buffer)?;
        }

        return Ok(packet_buffer);
    }

    /* pub fn create_response(&self, ttl: u32) -> DnsPacket {
        let mut response = DnsPacket {
            header: self.header.clone(),
            questions: self.questions.clone(),
            answers: Vec::new(),
        };

        response.header.is_response = true;
        response.header.recursion_available = true;
        response.header.recursion_desired = false;
        response.header.authoritative_answer = true;
        response.header.response_code = 0;
        response.header.answer_count = 1;

        response.answers = vec![DnsAnswer::new(
            self.questions.domain_names.clone(),
            self.questions.qclass.clone(),
            ttl,
            vec![
                DnsRecord::A {
                    address: "192.168.1.1".parse().unwrap(),
                }
            ],
        )];

        return response;
    } */
}
