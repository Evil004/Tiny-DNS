
pub mod serialize;
pub mod deserialize;


/* 
#[cfg(test)]
mod serialize_and_deserialize_tests {

    #[test]
    fn serialize_and_deserialize_qname() {
        
        let qname = "www.google.com".to_string();

        let vec = super::serialize::serialize_qname(qname.clone());

        let buf = vec.into_vec();
            
        let (_, qname) = super::deserialize::parse_qname((&buf,0)).unwrap();
        assert_eq!(qname, "www.google.com".to_string()); 
    }

    #[test]
    fn serialize_and_deserialize_16bits() {
        
        let number = 0x1234;

        let vec = super::serialize::serialize_16bits_to_bit_vec(number);

        let buf = vec.into_vec();
            
        let (_, number) = super::deserialize::take_16bits((&buf,0)).unwrap();
        assert_eq!(number, 0x1234); 
    }

    #[test]
    fn serialize_and_deserialize_1bit() {
        
        let number = 0b10101010;

        let vec = super::serialize::serialize_num_of_bits_u8_to_bit_vec(1, number);

        let buf = vec.into_vec();
            
        let (_, number) = super::deserialize::take_bits((&buf,0), 1).unwrap();
        assert_eq!(number, 0b0); 
    }

    #[test]
    fn serialize_and_deserialize_4bits() {
        
        let number = 0b10101010;

        let vec = super::serialize::serialize_num_of_bits_u8_to_bit_vec(4, number);

        let buf = vec.into_vec();
            
        let (_, number) = super::deserialize::take_bits((&buf,0), 4).unwrap();
        assert_eq!(number, 0b1010); 
    }
    
    #[test]
    fn serialize_and_deserialize_32bits() {
        
        let number = 600u32;

        let vec = super::serialize::serialize_32bits_to_bit_vec(number);

        let buf = vec.into_vec();

        let (_, number) = super::deserialize::take_bits((&buf,0), 32).unwrap();
    
        assert_eq!(number, 600);
    }
    
}
 */