use bitvec::{order::Msb0, vec::BitVec};

pub fn serialize_n_bits(n_bits: u8, number: u64) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let mut number = number;
    for _ in 0..n_bits {
        vec.push(number & 0b1 == 1);
        number >>= 1;
    }

    vec.reverse();

    return vec;
}

pub fn serialize_byte(num: u8) -> BitVec<u8, Msb0> {
    serialize_n_bits(8, num as u64)
}

pub trait Serialize {
    fn serialize(&self) -> BitVec<u8, Msb0>;
}
