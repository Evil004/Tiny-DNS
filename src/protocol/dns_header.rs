use nom::IResult;

use crate::parsing::{take_16bits, take_1bit_bool, take_3bits, take_4bits, BitInput};

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

    question_count: u16, // 16 bits
    answer_count: u16,   // 16 bits
    nscount: u16,        // 16 bits
    arcount: u16,        // 16 bits
}

impl DnsHeader {
    pub fn deserialize_header_from_bit_input(input: BitInput) -> IResult<BitInput, DnsHeader> {
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
