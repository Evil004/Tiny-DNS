use crate::parsing::Result;

use super::packet_buffer::PacketBuffer;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsHeader {
    id: u16,           // 16 bits
    is_response: bool, // 1  bit
    opcode: u8,        // 4  bits

    authoritative_answer: bool, // 1  bit
    truncated_message: bool,    // 1  bit
    recursion_desired: bool,    // 1  bit
    recursion_available: bool,  // 1  bit

    z: u8,             // 3  bit
    response_code: u8, // 4  bits

    pub question_count: u16, // 16 bits
    pub answer_count: u16,   // 16 bits
    nscount: u16,            // 16 bits
    arcount: u16,            // 16 bits
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
        let response_code = (flags & 0x0F) as u8;

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
            response_code,
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
        flags = (flags << 4) | (self.response_code as u16 & 0xF);

        packet_buffer.write_u16(flags);

        packet_buffer.write_u16(self.question_count);
        packet_buffer.write_u16(self.answer_count);
        packet_buffer.write_u16(self.nscount);
        packet_buffer.write_u16(self.arcount);

        return Ok(());
    }
}

#[cfg(test)]
mod dns_header_tests {
    use super::DnsHeader;

    use crate::protocol::packet_buffer;
    #[test]
    fn serialize_and_deserialize_dns_header() {
        let packet_buffer = &mut packet_buffer::PacketBuffer::new([0; 512]);
        let header = DnsHeader {
            id: 3241,
            is_response: true,
            opcode: 2,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: false,
            recursion_available: true,
            z: 3,
            response_code: 12,
            question_count: 1,
            answer_count: 1,
            nscount: 0,
            arcount: 0,
        };

        header.serialize(packet_buffer).unwrap();

        let result_header = DnsHeader::deserialize(packet_buffer).unwrap();

        assert_eq!(header.id, result_header.id);
        assert_eq!(header.is_response, result_header.is_response);
        assert_eq!(header.opcode, result_header.opcode);
        assert_eq!(
            header.authoritative_answer,
            result_header.authoritative_answer
        );
        assert_eq!(header.truncated_message, result_header.truncated_message);
        assert_eq!(header.recursion_desired, result_header.recursion_desired);
        assert_eq!(
            header.recursion_available,
            result_header.recursion_available
        );
        assert_eq!(header.z, result_header.z);
        assert_eq!(header.response_code, result_header.response_code);
        assert_eq!(header.question_count, result_header.question_count);
        assert_eq!(header.answer_count, result_header.answer_count);
        assert_eq!(header.nscount, result_header.nscount);
        assert_eq!(header.arcount, result_header.arcount);
    }

    /* #[test]
    fn set_as_response() {
        let mut header = DnsHeader {
            id: 3241,
            is_response: false,
            opcode: 2,
            authoritative_answer: true,
            truncated_message: true,
            recursion_desired: true,
            recursion_available: true,
            z: 3,
            response_code: 12,
            question_count: 1,
            answer_count: 0,
            nscount: 0,
            arcount: 0,
        };

        header.set_as_response(2);

        assert_eq!(header.is_response, true);
        assert_eq!(header.answer_count, 2);
    } */
}
