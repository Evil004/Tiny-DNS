use crate::errors::{DeserializeError, SerializeError};

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
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self, DeserializeError> {
        let header = DnsHeader::deserialize(packet_buffer);
        if let Err(_) = header {
            return Err(DeserializeError::InvalidHeader)
        }
        let header = header.unwrap();

        let questions = DnsQuery::deserialize(packet_buffer, header.question_count);
        if let Err(_) = questions {
            return Err(DeserializeError::InvalidHeader);
        }
        let questions = questions.unwrap();

        let mut answers = Vec::new();
        for _ in 0..header.answer_count {
            let answer = DnsRecord::deserialize(packet_buffer);
            if let Err(_) = answer {
                return Err(DeserializeError::InvalidRecord);
            }
            let answer = answer.unwrap();
            answers.push(answer);
        }

        let mut authority = Vec::new();
        for _ in 0..header.nscount {
            let name_server = DnsRecord::deserialize(packet_buffer);
            if let Err(_) = name_server {
                return Err(DeserializeError::InvalidRecord);
            }
            let name_server = name_server.unwrap();
            authority.push(name_server);
        }

        let mut additional = Vec::new();
        for _ in 0..header.arcount {
            let record = DnsRecord::deserialize(packet_buffer);
            if let Err(_) = record {
                return Err(DeserializeError::InvalidRecord);
            }
            let record = record.unwrap();
            additional.push(record);
        }

        return Ok(DnsPacket {
            header,
            questions,
            answers,
            authority,
            additional,
        });
    }

    pub fn serialize(&self) -> Result<PacketBuffer, SerializeError> {
        let mut packet_buffer = PacketBuffer::new([0u8; 512]);

        self.header.serialize(&mut packet_buffer)
            .map_err(|_| SerializeError::InvalidHeader)?;
        self.questions.serialize(&mut packet_buffer)
            .map_err(|_| SerializeError::InvalidQuestion)?;

        for answer in self.answers.iter() {
            answer.serialize(&mut packet_buffer)
                .map_err(|_| SerializeError::InvalidRecord)?;
        }

        for name_server in self.authority.iter() {
            name_server.serialize(&mut packet_buffer)
                .map_err(|_| SerializeError::InvalidRecord)?;
        }

        return Ok(packet_buffer);
    }
}
