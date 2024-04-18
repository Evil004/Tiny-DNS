use super::Result;

use super::{
    dns_record::DnsRecord, dns_header::DnsHeader, dns_query::DnsQuery, packet_buffer::PacketBuffer,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: DnsQuery,
    pub answers: Vec<DnsRecord>,
    pub authority: Vec<DnsRecord>,
    pub additional: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {
        let header = DnsHeader::deserialize(packet_buffer)?;

        let questions = DnsQuery::deserialize(packet_buffer, header.question_count)?;

        let mut answers = Vec::new();
        for _ in 0..header.answer_count {
            let answer = DnsRecord::deserialize(packet_buffer)?;
            answers.push(answer);
        }

        let mut authorization_name_servers = Vec::new();
        for _ in 0..header.nscount {
            let name_server = DnsRecord::deserialize(packet_buffer)?;
            authorization_name_servers.push(name_server);
        }

        let mut additional = Vec::new();
        for _ in 0..header.arcount {
            let record = DnsRecord::deserialize(packet_buffer);

            if  let Ok(record) = record {
                additional.push(record);
            }
        }

        return Ok(DnsPacket {
            header,
            questions,
            answers,
            authority: authorization_name_servers,
            additional,
        });
    }

    pub fn serialize(&self) -> Result<PacketBuffer> {
        let mut packet_buffer = PacketBuffer::new([0u8; 512]);

        self.header.serialize(&mut packet_buffer)?;
        self.questions.serialize(&mut packet_buffer)?;

        for answer in self.answers.iter() {
            answer.serialize(&mut packet_buffer)?;
        }

        for name_server in self.authority.iter() {
            name_server.serialize(&mut packet_buffer)?;
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
