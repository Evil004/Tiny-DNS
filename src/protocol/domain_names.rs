use bitvec::{order::Msb0, vec::BitVec};
use nom::{combinator::peek, complete::take, multi::count, IResult};

use crate::parsing::{
    deserialize::{BitInput, Deserialize},
    serialize::{serialize_byte, serialize_n_bits, Serialize},
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DomainNames {
    labels_array: Vec<DomainParts>,
}

#[allow(dead_code)]
impl DomainNames {
    pub fn new_from_vec(labels_array: Vec<DomainParts>) -> DomainNames {
        return DomainNames { labels_array };
    }

    pub fn get_labels(&self) -> Vec<DomainParts> {
        return self.labels_array.clone();
    }

    pub fn get_total_len(&self) -> usize {
        let mut total_len = 0;
        for label in self.labels_array.iter() {
            total_len += label.get_size() as usize
        }
        return total_len;
    }

    pub fn get_domains(&self) -> Vec<(String, u16)> {
        let mut domains = Vec::new();
        let mut domain = String::new();
        let mut start_pos = 0;

        for (i, label) in self.labels_array.iter().enumerate() {
            if domain.len() == 0 {
                start_pos = self.get_pos_from_index(i);
            }
            match label {
                DomainParts::Pointer { pos } => {
                    domain.push_str(&self.get_domain_from_pos(*pos));
                    domains.push((domain.clone(), start_pos));
                    domain.clear();
                }
                DomainParts::Label { len: _, string } => {
                    domain.push_str(&format!("{}.", string));
                }
                DomainParts::End => {
                    domain.pop();
                    domains.push((domain.clone(), start_pos));
                    domain.clear();
                }
            }
        }

        return domains;
    }

    fn get_pos_from_index(&self, index: usize) -> u16 {
        let mut reading_pos = 0;

        for (i, label) in self.labels_array.iter().enumerate() {
            if i == index {
                return reading_pos;
            }
            reading_pos += label.get_size();
        }

        return reading_pos;
    }

    fn get_domain_from_pos(&self, pos: u16) -> String {
        let mut domain = String::new();

        let labels = self.get_from_pos(pos);
        for label in labels {
            match label {
                DomainParts::Pointer { pos } => {
                    domain.push_str(&self.get_domain_from_pos(*pos));
                    return domain;
                }
                DomainParts::Label { len: _, string } => {
                    domain.push_str(&format!("{}.", string));
                }
                DomainParts::End => {
                    domain.pop();
                    return domain;
                }
            }
        }

        return domain;
    }

    fn get_from_pos(&self, pos: u16) -> &[DomainParts] {
        let mut reading_pos = 0;

        for (i, label) in self.labels_array.iter().enumerate() {
            if pos > reading_pos {
                reading_pos += label.get_size()
            } else {
                return &self.labels_array[i..];
            }
        }

        return &[];
    }
}

impl DomainNames {
    pub fn deserialize(input: BitInput, num_of_domains: u16) -> IResult<BitInput, DomainNames> {
        let mut domain_count = 0;
        let mut parts: Vec<DomainParts> = Vec::new();
        let mut final_input = input;

        /* while domain_count < num_of_domains as usize {
            let (input, part) = DomainParts::deserialize(final_input)?;

            final_input = input;

            if let DomainParts::End = part {
                domain_count += 1
            }

            parts.push(part);
        } */

        let domain_names = DomainNames::new_from_vec(parts);
        return Ok((final_input, domain_names));
    }
}

impl Serialize for DomainNames {
    fn serialize(&self) -> BitVec<u8, Msb0> {
        let mut vec: BitVec<u8, Msb0> = BitVec::new();
        let labels = self.get_labels();

        for label in labels {
            match label {
                DomainParts::Pointer { pos } => {
                    vec.append(&mut serialize_n_bits(2, 0b11));
                    vec.append(&mut serialize_n_bits(14, pos as u64));
                }
                DomainParts::Label { len, string } => {
                    vec.append(&mut serialize_byte(len));
                    for c in string.as_bytes() {
                        vec.append(&mut serialize_byte(*c));
                    }
                }
                DomainParts::End => {
                    vec.append(&mut serialize_byte(0));
                }
            }
        }

        return vec;
    }
}

#[derive(Clone, Debug)]
pub enum DomainParts {
    Pointer { pos: u16 },
    Label { len: u8, string: String },
    End,
}

impl DomainParts {
    pub fn get_size(&self) -> u16 {
        match self {
            DomainParts::Pointer { .. } => 2,
            DomainParts::Label { len, .. } => *len as u16 + 1,
            DomainParts::End => 1,
        }
    }
}
/* 
impl Deserialize for DomainParts {
    fn deserialize(input: BitInput) -> IResult<BitInput, Self>
    where
        Self: Sized,
    {
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
}
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_domain() {
        let domain_names = DomainNames {
            labels_array: vec![
                DomainParts::Label {
                    len: 3,
                    string: String::from("www"),
                },
                DomainParts::Label {
                    len: 6,
                    string: String::from("google"),
                },
                DomainParts::Label {
                    len: 3,
                    string: String::from("com"),
                },
                DomainParts::End,
            ],
        };

        let domains = domain_names.get_domains();

        assert_eq!(domains, vec![(String::from("www.google.com"), 0u16)]);
    }

    #[test]
    fn test_get_domain_wiht_pointer() {
        let domain_names = DomainNames {
            labels_array: vec![
                DomainParts::Label {
                    len: 3,
                    string: String::from("www"),
                },
                DomainParts::Label {
                    len: 6,
                    string: String::from("google"),
                },
                DomainParts::Label {
                    len: 3,
                    string: String::from("com"),
                },
                DomainParts::End,
                DomainParts::Label {
                    len: 6,
                    string: String::from("images"),
                },
                DomainParts::Pointer { pos: 4 },
            ],
        };

        let domains = domain_names.get_domains();

        assert_eq!(
            domains,
            vec![
                (String::from("www.google.com"), 0u16),
                (String::from("images.google.com"), 16u16)
            ]
        );
    }

    #[test]
    fn test_get_domain_with_pointer_to_pointer() {
        let domain_names = DomainNames {
            labels_array: vec![
                DomainParts::Label {
                    len: 3,
                    string: String::from("www"),
                },
                DomainParts::Label {
                    len: 6,
                    string: String::from("google"),
                },
                DomainParts::Label {
                    len: 3,
                    string: String::from("com"),
                },
                DomainParts::End,
                DomainParts::Label {
                    len: 6,
                    string: String::from("images"),
                },
                DomainParts::Pointer { pos: 4 },
                DomainParts::Label {
                    len: 4,
                    string: String::from("test"),
                },
                DomainParts::Pointer { pos: 16 },
            ],
        };

        let domains = domain_names.get_domains();

        assert_eq!(
            domains,
            vec![
                (String::from("www.google.com"), 0u16),
                (String::from("images.google.com"), 16u16),
                (String::from("test.images.google.com"), 25u16)
            ]
        );
    }

    #[test]
    fn serialize_and_deserialize_qname() {
        let domain_names = DomainNames::new_from_vec(vec![
            DomainParts::Label {
                len: 3,
                string: String::from("www"),
            },
            DomainParts::Label {
                len: 6,
                string: String::from("google"),
            },
            DomainParts::Label {
                len: 3,
                string: String::from("com"),
            },
            DomainParts::End,
        ]);

        let vec = domain_names.serialize();
        let buf = vec.into_vec();

        let (res, des_domain_names) = DomainNames::deserialize((&buf, 0), 1).unwrap();

        let domains = domain_names.get_domains();
        let des_domains = des_domain_names.get_domains();

        assert_eq!(domains, des_domains);
        assert_eq!(res, (&[] as &[u8], 0));
    }
}
