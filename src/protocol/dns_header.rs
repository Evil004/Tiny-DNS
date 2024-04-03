use bitvec::{order::Msb0, vec::BitVec};
use nom::IResult;

use crate::parsing::{
    deserialize::{take_16bits, take_1bit_bool, take_3bits, take_4bits, BitInput, Deserialize},
    serialize::{serialize_16bits_to_bit_vec, serialize_num_of_bits_u8_to_bit_vec, Serialize},
};

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

    z: u8,             // 4  bit
    response_code: u8, // 4  bits

    pub question_count: u16, // 16 bits
    answer_count: u16,   // 16 bits
    nscount: u16,        // 16 bits
    arcount: u16,        // 16 bits
}

impl DnsHeader {
    pub fn set_as_response(&mut self, num_answers: u16) {
        self.is_response = true;
        self.answer_count = num_answers;
    }
}

impl Serialize for DnsHeader {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();
        vec.append(&mut serialize_16bits_to_bit_vec(self.id));
        vec.push(self.is_response);
        vec.append(&mut serialize_num_of_bits_u8_to_bit_vec(4, self.opcode));
        vec.push(self.authoritative_answer);
        vec.push(self.truncated_message);
        vec.push(self.recursion_desired);
        vec.push(self.recursion_available);
        vec.append(&mut serialize_num_of_bits_u8_to_bit_vec(3, self.z));
        vec.append(&mut serialize_num_of_bits_u8_to_bit_vec(
            4,
            self.response_code,
        ));
        vec.append(&mut serialize_16bits_to_bit_vec(self.question_count));
        vec.append(&mut serialize_16bits_to_bit_vec(self.answer_count));
        vec.append(&mut serialize_16bits_to_bit_vec(self.nscount));
        vec.append(&mut serialize_16bits_to_bit_vec(self.arcount));

        return vec;
    }
}

impl Deserialize for DnsHeader {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self> {
        let (input, id) = take_16bits(input)?;
        let (input, is_response) = take_1bit_bool(input)?;

        let (input, opcode) = take_4bits(input)?;

        let (input, authoritative_answer) = take_1bit_bool(input)?;
        let (input, truncated_message) = take_1bit_bool(input)?;

        let (input, recursion_desired) = take_1bit_bool(input)?;
        let (input, recursion_available) = take_1bit_bool(input)?;

        let (input, z) = take_3bits(input)?;
        let (input, response_code) = take_4bits(input)?;
        let (input, question_count) = take_16bits(input)?;
        let (input, answer_count) = take_16bits(input)?;
        let (input, nscount) = take_16bits(input)?;
        let (input, arcount) = take_16bits(input)?;

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

        return Ok((input, header));
    }
}

#[cfg(test)]
mod dns_header_tests{
    use crate::parsing::serialize::Serialize;
    use crate::parsing::deserialize::Deserialize;
    use super::DnsHeader;
    #[test]
    fn serialize_and_deserialize_dns_header() {
        

        let header = DnsHeader {
            id: 3241,
            is_response: true,
            opcode: 2,
            authoritative_answer: true,
            truncated_message: true,
            recursion_desired: true,
            recursion_available: true,
            z: 3,
            response_code: 12,
            question_count: 1,
            answer_count: 1,
            nscount: 0,
            arcount: 0,
        };

        let vec = header.serialize();
        let buf = vec.into_vec();

        let (_, result_header) = DnsHeader::deserialize((&buf, 0)).unwrap();

        assert_eq!(header.id, result_header.id);
        assert_eq!(header.is_response, result_header.is_response);
        assert_eq!(header.opcode, result_header.opcode);
        assert_eq!(header.authoritative_answer, result_header.authoritative_answer);
        assert_eq!(header.truncated_message, result_header.truncated_message);
        assert_eq!(header.recursion_desired, result_header.recursion_desired);
        assert_eq!(header.recursion_available, result_header.recursion_available);
        assert_eq!(header.z, result_header.z);
        assert_eq!(header.response_code, result_header.response_code);
        assert_eq!(header.question_count, result_header.question_count);
        assert_eq!(header.answer_count, result_header.answer_count);
        assert_eq!(header.nscount, result_header.nscount);
        assert_eq!(header.arcount, result_header.arcount);

    }

    #[test]
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
    }
}