use super::Result;

use super::packet_buffer::PacketBuffer;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DnsHeader {
    pub id: u16,           // 16 bits
    pub is_response: bool, // 1  bit
    pub opcode: u8,        // 4  bits

    pub authoritative_answer: bool, // 1  bit
    pub truncated_message: bool,    // 1  bit
    pub recursion_desired: bool,    // 1  bit
    pub recursion_available: bool,  // 1  bit

    pub z: u8,             // 3  bit
    pub rcode: ResponseCode, // 4  bits

    pub question_count: u16, // 16 bits
    pub answer_count: u16,   // 16 bits
    pub nscount: u16,        // 16 bits
    pub arcount: u16,        // 16 bits
}

impl DnsHeader {
    pub fn deserialize(packet_buffer: &mut PacketBuffer) -> Result<Self> {
        let id = packet_buffer.read_u16()?;
        let flags = packet_buffer.read_u16()?;

        let is_response = (flags >> 15) > 0;
        let opcode = (flags >> 11 & 0x0F) as u8;
        let authoritative_answer = (flags >> 10 & 1) > 0;
        let truncated_message = (flags >> 9 & 1) > 0;
        let recursion_desired = (flags >> 8 & 1) > 0;
        let recursion_available = (flags >> 7 & 1) > 0;
        let z = (flags >> 4 & 0x07) as u8;
        let response_code = ResponseCode::from_u8((flags & 0x0F) as u8).unwrap();

        let question_count = packet_buffer.read_u16()?;
        let answer_count = packet_buffer.read_u16()?;
        let nscount = packet_buffer.read_u16()?;
        let arcount = packet_buffer.read_u16()?;

        let header = DnsHeader {
            id,
            is_response,
            opcode,
            authoritative_answer,
            truncated_message,
            recursion_desired,
            recursion_available,
            z,
            rcode: response_code,
            question_count,
            answer_count,
            nscount,
            arcount,
        };

        return Ok(header);
    }

    pub fn serialize(&self, packet_buffer: &mut PacketBuffer) -> Result<()> {
        packet_buffer.write_u16(self.id);

        let mut flags: u16;

        flags = if self.is_response { 1 } else { 0 };

        flags = (flags << 4) | (self.opcode as u16 & 0xF);
        flags = (flags << 1) | (if self.authoritative_answer { 1 } else { 0 } & 0b1);
        flags = (flags << 1) | (if self.truncated_message { 1 } else { 0 } & 0b1);
        flags = (flags << 1) | (if self.recursion_desired { 1 } else { 0 } & 0b1);
        flags = (flags << 1) | (if self.recursion_available { 1 } else { 0 } & 0b1);

        flags = (flags << 3) | (self.z as u16 & 0x7);
        flags = (flags << 4) | (self.rcode as u16 & 0xF);

        packet_buffer.write_u16(flags);

        packet_buffer.write_u16(self.question_count);
        packet_buffer.write_u16(self.answer_count);
        packet_buffer.write_u16(self.nscount);
        packet_buffer.write_u16(self.arcount);

        return Ok(());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NXDomain = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl ResponseCode {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(ResponseCode::NoError),
            1 => Ok(ResponseCode::FormatError),
            2 => Ok(ResponseCode::ServerFailure),
            3 => Ok(ResponseCode::NXDomain),
            4 => Ok(ResponseCode::NotImplemented),
            5 => Ok(ResponseCode::Refused),
            _ => Err("Invalid ResponseCode".into()),
        }
    }
}