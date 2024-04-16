use std::io::Error;

pub struct PacketBuffer {
    pub buffer: [u8; 512],
    pub pos: usize,
}

impl PacketBuffer {
    fn new(buffer: [u8; 512]) -> Self {
        PacketBuffer { buffer, pos: 0 }
    }
    fn get(&self) -> Result<u8, Error> {
        return Ok(self.buffer[self.pos]);
    }

    pub fn read(&mut self) -> Result<u8, Error> {
        let num = self.buffer[self.pos];
        self.pos += 1;

        return Ok(num);
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let num = ((self.read()? as u16) << 8) | self.read()? as u16;
        return Ok(num);
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
}
