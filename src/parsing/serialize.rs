use bitvec::{order::Msb0, vec::BitVec};

use crate::protocol::domain_names::{DomainNames, Next};

pub fn serialize_num_of_bits_u8_to_bit_vec(
    num_of_bits_to_read: u8,
    number: u8,
) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let mut number = number;
    for _ in 0..num_of_bits_to_read {
        vec.push(number & 0b1 == 1);
        number >>= 1;
    }

    vec.reverse();

    return vec;
}
pub fn serialize_num_of_bits_u16_to_bit_vec(
    num_of_bits_to_read: u8,
    number: u16,
) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let mut number = number;
    for _ in 0..num_of_bits_to_read {
        vec.push(number & 0b1 == 1);
        number >>= 1;
    }

    vec.reverse();

    return vec;
}

pub fn serialize_byte(num: u8) -> BitVec<u8, Msb0> {
    serialize_num_of_bits_u8_to_bit_vec(8, num)
}

pub fn serialize_14last_bits_to_bit_vec(number: u16) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let mut number = number;
    for _ in 0..14 {
        vec.push(number & 0b1 == 1);
        number >>= 1;
    }

    vec.reverse();

    return vec;
}

pub fn serialize_16bits_to_bit_vec(number: u16) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let mut number = number;
    for _ in 0..16 {
        vec.push(number & 0b1 == 1);
        number >>= 1;
    }

    vec.reverse();

    return vec;
}

pub fn serialize_32bits_to_bit_vec(number: u32) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let mut number = number;
    for _ in 0..32 {
        vec.push(number & 0b1 == 1);
        number >>= 1;
    }

    vec.reverse();

    return vec;
}
pub fn serialize_domain_names(domain_names: DomainNames) -> BitVec<u8, Msb0> {
    let mut vec: BitVec<u8, Msb0> = BitVec::new();
    let labels = domain_names.get_labels();

    for label in labels {
        vec.append(&mut serialize_byte(label.len));
        for c in label.string.chars() {
            vec.append(&mut serialize_num_of_bits_u8_to_bit_vec(8, c as u8));
        }

       match label.next {
            Next::End => {
                vec.append(&mut serialize_byte(0b0));
            }
            Next::Label => {}
            Next::Pointer { pos } => {
                vec.push(true);
                vec.push(true);

                let mut pos = serialize_14last_bits_to_bit_vec(pos);
                vec.append(&mut pos);

                break;
            }
        }
    }

    vec.append(&mut serialize_byte(0b0));

    return vec;
}

pub trait Serialize {
    fn serialize(&self) -> BitVec<u8, Msb0>;
}
