use nom::{bits::complete::tag, bits::complete::take, multi::many_till, IResult};

pub type BitInput<'a> = (&'a [u8], usize);

pub fn take_bits(input: BitInput, num_bits: usize) -> IResult<BitInput, u64> {
    let (input, num) = take(num_bits)(input)?;
    Ok((input, num))
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

pub fn parse_qname(input: BitInput) -> IResult<BitInput, String> {
    let (input, domain_names) = many_till(parse_domain, byte0)(input)?;

    let domain: String = domain_names
        .0
        .into_iter()
        .filter(|name| !name.is_empty())
        .collect::<Vec<String>>()
        .join(".");

    return Ok((input, domain));
}

pub fn byte0(input: BitInput) -> IResult<BitInput, ()> {
    let (input, _) = tag(0b00000000, 8u8)(input)?;

    return Ok((input, ()));
}

pub fn parse_domain(input: BitInput) -> IResult<BitInput, String> {
    let (input, num): (BitInput, u8) = take(8usize)(input)?;

    let (input, chars_in_u64): (BitInput, u64) = take(num * 8)(input)?;

    let chars: Vec<char> = chars_in_u64
        .to_be_bytes()
        .iter()
        .filter(|x| **x != 0)
        .map(|x| *x as char)
        .collect();

    let domain: String = chars.into_iter().collect();

    return Ok((input, domain));
}
