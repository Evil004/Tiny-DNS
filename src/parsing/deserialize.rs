use nom::{bits::complete::take, combinator::peek, IResult};

use crate::protocol::domain_names::{DomainNames, Label, Next};

pub type BitInput<'a> = (&'a [u8], usize);

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
    let mut labels = Vec::new();
    let mut final_input = input;

    while domain_count < num_of_domains as usize {
        dbg!(labels.clone());
        let (input, label): (BitInput, Label) = parse_label(final_input)?;
        final_input = input;
        labels.push(label.clone());
        if let Next::End = label.next {
            domain_count += 1;
        }
    }

    labels.pop();

    let domain_names = DomainNames::new(labels);
    return Ok((final_input, domain_names));
}

pub fn parse_label(input: BitInput) -> IResult<BitInput, Label> {
    dbg!(input.clone());
    let (input, chars_count): (BitInput, u8) = take(8usize)(input)?;

    if chars_count == 0 {
        return Ok((input, Label::new(0, "".to_string(), Next::End)));
    }

    let (input, chars_in_u64): (BitInput, u64) = take(chars_count * 8)(input)?;

    let chars: Vec<char> = chars_in_u64
        .to_be_bytes()
        .iter()
        .filter(|x| **x != 0)
        .map(|x| *x as char)
        .collect();

    let label_string: String = chars.into_iter().collect();

    let (input, next): (BitInput, Next) = get_next(input)?;

    let label = Label::new(chars_count, label_string, next);

    return Ok((input, label));
}

fn get_next(input: BitInput) -> IResult<BitInput, Next> {
    let (_, next_bits): (BitInput, u8) = peek(take(8u8))(input)?;
    if next_bits == 0b0 {
        let (input, _): (BitInput, u8) = take(8u8)(input)?;
        return Ok((input, Next::End));
    }

    let (_, next_bits): (BitInput, u8) = peek(take(2u8))(input)?;
    if next_bits == 0b0 {
        return Ok((input, Next::Label));
    }

    let (input, _): (BitInput, u8) = take(2u8)(input)?;

    let (input, pos): (BitInput, u16) = take(14u8)(input)?;

    return Ok((input, Next::Pointer { pos }));
}

pub trait Deserialize {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self>
    where
        Self: Sized;
}
