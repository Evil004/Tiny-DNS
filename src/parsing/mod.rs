use nom::bits::complete::take;
use nom::IResult;

pub type BitInput<'a> = (&'a [u8], usize);

pub fn take_16bits(input: BitInput) -> IResult<BitInput, u16> {
    let (input, num) = take(16u8)(input)?;
    return Ok((input, num));
}

pub fn take_4bits(input: BitInput) -> IResult<BitInput, u8> {
    let (input, num) = take(4u8)(input)?;
    return Ok((input, num));
}
pub fn take_3bits(input: BitInput) -> IResult<BitInput, u8> {
    let (input, num) = take(3u8)(input)?;
    return Ok((input, num));
}
pub fn take_1bit_bool(input: BitInput) -> IResult<BitInput, bool> {
    let (input, num): (BitInput, u8) = take(1usize)(input)?;

    return Ok((input, num == 1));
}
