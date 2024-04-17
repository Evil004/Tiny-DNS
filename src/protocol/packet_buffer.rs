use super::Result;

#[derive(Debug)]
pub struct PacketBuffer {
    pub buffer: [u8; 512],
    pub pos: usize,
}

impl PacketBuffer {
    pub fn new(buffer: [u8; 512]) -> Self {
        PacketBuffer { buffer, pos: 0 }
    }

    fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn peek(&self) -> Result<u8> {
        return Ok(self.buffer[self.pos]);
    }

    pub fn read(&mut self) -> Result<u8> {
        let num = self.buffer[self.pos];
        self.pos += 1;

        return Ok(num);
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let num = ((self.read()? as u16) << 8) | self.read()? as u16;

        return Ok(num);
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let num = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | self.read()? as u32;

        return Ok(num);
    }

    pub fn read_qname(&mut self) -> Result<String> {
        let mut higger_pos = self.pos;

        let mut actual_domain = String::new();

        loop {
            if self.peek()? == 0 {
                self.pos += 1;
                actual_domain.pop();
                break;
            }

            if self.peek()? == 0xC0 {
                // Read the pointer and remove the first two bits that are always 1
                let pointer = self.read_u16()? & 0x3FFF;

                if pointer as usize >= self.buffer.len() {
                    return Err("Pointer out of bounds".to_string().into());
                }

                if higger_pos < self.pos {
                    higger_pos = self.pos;
                }

                self.seek(pointer as usize);
                continue;
            }

            let label_length = self.read()? as usize;

            if label_length > 63 {
                return Err("Label length too long".to_string().into());
            }

            for _ in 0..label_length {
                actual_domain.push(self.read()? as char);
            }

            actual_domain.push('.');
        }

        if higger_pos > self.pos {
            self.seek(higger_pos);
        }

        return Ok(actual_domain);
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        for _ in 0..length {
            bytes.push(self.read()?);
        }

        return Ok(bytes);
    }

    pub fn write(&mut self, num: u8) {
        self.buffer[self.pos] = num;
        self.pos += 1;
    }

    pub fn write_u16(&mut self, num: u16) {
        self.write((num >> 8) as u8);
        self.write(num as u8);
    }

    pub fn write_u32(&mut self, num: u32) {
        self.write((num >> 24) as u8);
        self.write((num >> 16) as u8);
        self.write((num >> 8) as u8);
        self.write(num as u8);
    }

    pub fn write_qname(&mut self, domain: &str) {
        for label in domain.split('.') {
            self.write(label.len() as u8);

            
        }

        self.write(0);
    }

    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        for byte in bytes {
            self.write(byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_buffer() {
        let mut buffer = PacketBuffer::new([0; 512]);
        buffer.buffer[0] = 0b00000001;
        buffer.buffer[1] = 0b00000010;
        assert_eq!(buffer.read_u16().unwrap(), 258);
    }

    #[test]
    fn test_qname() {
        let mut buffer = PacketBuffer::new([0; 512]);
        let mut tmp_vec: Vec<u8> = Vec::new();

        tmp_vec.push(3);
        tmp_vec.push('w' as u8);
        tmp_vec.push('w' as u8);
        tmp_vec.push('w' as u8);
        tmp_vec.push(6);
        tmp_vec.push('g' as u8);
        tmp_vec.push('o' as u8);
        tmp_vec.push('o' as u8);
        tmp_vec.push('g' as u8);
        tmp_vec.push('l' as u8);
        tmp_vec.push('e' as u8);
        tmp_vec.push(3);
        tmp_vec.push('c' as u8);
        tmp_vec.push('o' as u8);
        tmp_vec.push('m' as u8);
        tmp_vec.push(0);
        tmp_vec.push(6);
        tmp_vec.push('i' as u8);
        tmp_vec.push('m' as u8);
        tmp_vec.push('a' as u8);
        tmp_vec.push('g' as u8);
        tmp_vec.push('e' as u8);
        tmp_vec.push('s' as u8);
        tmp_vec.push(0xC0);
        tmp_vec.push(0x04);
        tmp_vec.push(3);
        tmp_vec.push('w' as u8);
        tmp_vec.push('w' as u8);
        tmp_vec.push('w' as u8);
        tmp_vec.push(0xC0);
        tmp_vec.push(16);

        for i in 0..tmp_vec.len() {
            buffer.buffer[i] = tmp_vec[i];
        }

        assert_eq!(buffer.read_qname().unwrap(), "www.google.com".to_string());
        assert_eq!(
            buffer.read_qname().unwrap(),
            "images.google.com".to_string()
        );
        assert_eq!(
            buffer.read_qname().unwrap(),
            "www.images.google.com".to_string()
        );
    }
}
