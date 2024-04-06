use nom::{bits::complete::take, multi::count, IResult};

pub type BitInput<'a> = (&'a [u8], usize);

pub fn take_n_bits(input: BitInput, num_bits: usize) -> IResult<BitInput, u64> {
    take(num_bits)(input)
}

#[allow(dead_code)]
pub fn take_32bits(input: BitInput) -> IResult<BitInput, u32> {
    take_n_bits(input, 32).map(|(input, num)| (input, num as u32))
}

pub fn take_16bits(input: BitInput) -> IResult<BitInput, u16> {
    take_n_bits(input, 16).map(|(input, num)| (input, num as u16))
}

pub fn take_4bits(input: BitInput) -> IResult<BitInput, u8> {
    take_n_bits(input, 4).map(|(input, num)| (input, num as u8))
}

pub fn take_3bits(input: BitInput) -> IResult<BitInput, u8> {
    take_n_bits(input, 3).map(|(input, num)| (input, num as u8))
}

pub fn take_1bit_bool(input: BitInput) -> IResult<BitInput, bool> {
    take_n_bits(input, 1).map(|(input, num)| (input, num == 1))
}

pub fn take_vec_of_n_bytes(input: BitInput, num_bytes: u16) -> IResult<BitInput, Vec<u8>> {
    count(take(8usize), num_bytes as usize)(input)
}

pub trait Deserialize {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self>
    where
        Self: Sized;
}
pub trait DeserializeWithLength {
    fn deserialize(input: BitInput, length: u16) -> IResult<BitInput, Self>
    where
        Self: Sized;
    
}