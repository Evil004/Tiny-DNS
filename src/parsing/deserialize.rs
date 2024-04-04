use nom::{bits::complete::take, combinator::peek, multi::count, IResult};

use crate::protocol::domain_names::{DomainNames, DomainParts};

pub type BitInput<'a> = (&'a [u8], usize);

// TODO: Refactor all this code

pub fn take_bits(input: BitInput, num_bits: usize) -> IResult<BitInput, u64> {
    let (input, num) = take(num_bits)(input)?;
    Ok((input, num))
}

#[allow(dead_code)]
pub fn take_32bits(input: BitInput) -> IResult<BitInput, u32> {
    take_bits(input, 32).map(|(input, num)| (input, num as u32))
}

pub fn take_16bits(input: BitInput) -> IResult<BitInput, u16> {
    take_bits(input, 16).map(|(input, num)| (input, num as u16))
}

pub fn take_4bits(input: BitInput) -> IResult<BitInput, u8> {
    take_bits(input, 4).map(|(input, num)| (input, num as u8))
}

pub fn take_3bits(input: BitInput) -> IResult<BitInput, u8> {
    take_bits(input, 3).map(|(input, num)| (input, num as u8))
}

pub fn take_1bit_bool(input: BitInput) -> IResult<BitInput, bool> {
    take_bits(input, 1).map(|(input, num)| (input, num == 1))
}

pub fn deserialize_domain_names(
    input: BitInput,
    num_of_domains: u16,
) -> IResult<BitInput, DomainNames> {
    let mut domain_count = 0;
    let mut parts: Vec<DomainParts> = Vec::new();
    let mut final_input = input;

    while domain_count < num_of_domains as usize {
        let (input, part) = parse_part(final_input)?;
        final_input = input;
        if let DomainParts::End = part {
            domain_count += 1
        }
        parts.push(part);
    }

    let domain_names = DomainNames::new_from_vec(parts);
    return Ok((final_input, domain_names));
}

fn parse_part(input: BitInput) -> IResult<BitInput, DomainParts> {
    let (_, next_bits): (BitInput, u8) = peek(take(8u8))(input)?;
    if next_bits == 0b0 {
        let (input, _): (BitInput, u8) = take(8u8)(input)?;
        return Ok((input, DomainParts::End));
    }

    let (_, next_bits): (BitInput, u8) = peek(take(2u8))(input)?;
    if next_bits == 0b0 {
        let (input, char_count): (BitInput, u8) = take(8u8)(input)?;
        let (input, chars): (BitInput, Vec<u8>) =
            count(take((8) as u16), char_count as usize)(input)?;

        return Ok((
            input,
            DomainParts::Label {
                len: char_count,
                string: chars.iter().map(|c| *c as char).collect::<String>(),
            },
        ));
    }

    let (input, _): (BitInput, u8) = take(2u8)(input)?;

    let (input, pos): (BitInput, u16) = take(14u8)(input)?;

    return Ok((input, DomainParts::Pointer { pos }));
}

pub trait Deserialize {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self>
    where
        Self: Sized;
}
