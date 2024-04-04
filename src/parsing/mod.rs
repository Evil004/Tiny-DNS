pub mod deserialize;
pub mod serialize;

#[cfg(test)]
mod serialize_and_deserialize_tests {

    use crate::protocol::domain_names::{DomainNames, DomainParts};

    #[test]
    fn serialize_and_deserialize_qname() {
        let domain_names = DomainNames::new_from_vec(
            vec![
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
            ]
        );

        let vec = super::serialize::serialize_domain_names(domain_names.clone());
        let buf = vec.into_vec();

        let (res, des_domain_names) =
            super::deserialize::deserialize_domain_names((&buf, 0), 1).unwrap();

        let domains = domain_names.get_domains();
        let des_domains = des_domain_names.get_domains();

        assert_eq!(domains, des_domains);
        assert_eq!(res, (&[] as &[u8], 0));
    }

    #[test]
    fn serialize_and_deserialize_16bits() {
        let number = 0x1234;

        let vec = super::serialize::serialize_16bits_to_bit_vec(number);

        let buf = vec.into_vec();

        let (_, number) = super::deserialize::take_16bits((&buf, 0)).unwrap();
        assert_eq!(number, 0x1234);
    }

    #[test]
    fn serialize_and_deserialize_1bit() {
        let number = 0b10101010;

        let vec = super::serialize::serialize_num_of_bits_u8_to_bit_vec(1, number);

        let buf = vec.into_vec();

        let (_, number) = super::deserialize::take_bits((&buf, 0), 1).unwrap();
        assert_eq!(number, 0b0);
    }

    #[test]
    fn serialize_and_deserialize_4bits() {
        let number = 0b10101010;

        let vec = super::serialize::serialize_num_of_bits_u8_to_bit_vec(4, number);

        let buf = vec.into_vec();

        let (_, number) = super::deserialize::take_bits((&buf, 0), 4).unwrap();
        assert_eq!(number, 0b1010);
    }

    #[test]
    fn serialize_and_deserialize_32bits() {
        let number = 600u32;

        let vec = super::serialize::serialize_32bits_to_bit_vec(number);

        let buf = vec.into_vec();

        let (_, number) = super::deserialize::take_bits((&buf, 0), 32).unwrap();

        assert_eq!(number, 600);
    }
}
