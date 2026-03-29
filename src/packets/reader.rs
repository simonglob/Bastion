const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

#[derive(Clone, Copy)]
pub struct Reader<'a> {
    buffer: &'a [u8],
    pos: usize
}

impl<'a> Reader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Reader { buffer: buffer, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.buffer[self.pos..].len() <= 0
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn get_buffer(&self) -> &'a [u8] {
        self.buffer
    }

    pub fn peek(&mut self, size: usize) {
        self.pos += size;
    }

    pub fn read_u8(&mut self) -> u8 {
        let ret = self.buffer[self.pos];
        self.peek(1);
        ret
    }

    pub fn read_u16(&mut self) -> u16 {
        let ret = u16::from_be_bytes([self.buffer[self.pos], self.buffer[self.pos + 1]]);
        self.peek(2);
        ret
    }

    pub fn read_i64(&mut self) -> i64 {
        let ret = i64::from_be_bytes(self.buffer[self.pos..self.pos+8].try_into().unwrap());
        self.peek(8);
        ret
    }

    pub fn read_varint(&mut self) -> i32 {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let current_byte = self.read_u8();
            value |= ((current_byte & SEGMENT_BITS) as i32) << shift;

            if (current_byte & CONTINUE_BIT) == 0{
                break;
            }

            shift += 7;

            if shift >= 32 {
                panic!("VarInt is too big")
            }
        }

        value
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_varint() as usize;
        let string_bytes = &self.buffer[self.pos..self.pos + length];
        self.peek(length);
        
        String::from_utf8(string_bytes.to_vec()).expect("Invalid UTF-8")
    }
}