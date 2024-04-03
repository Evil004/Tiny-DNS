pub mod deserialize;
pub mod serialize;

#[cfg(test)]
mod serialize_and_deserialize_tests {

    use crate::protocol::domain_names::{DomainNames, Label, Next};

    #[test]
    fn serialize_and_deserialize_qname() {
        let domain_names = DomainNames::new(vec![
            Label::new(3, "www".to_string(), Next::Label),
            Label::new(6, "google".to_string(), Next::Label),
            Label::new(3, "com".to_string(), Next::End),
            Label::new(6, "images".to_string(), Next::Pointer { pos: 4 }),
        ]);

        let vec = super::serialize::serialize_domain_names(domain_names.clone());

        let buf = vec.into_vec();

        let (_, des_domain_names) =
            super::deserialize::deserialize_domain_names((&buf, 0), 2).unwrap();

        let domains = domain_names.get_domains();
        let des_domains = des_domain_names.get_domains();

        assert_eq!(domains, des_domains);
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
