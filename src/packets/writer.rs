pub struct Writer {
    content: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self { content: vec![] }
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    pub fn write_u8(&mut self, i: u8) {
        self.content.push(i);
    }

    pub fn write(&mut self, data: &[u8]) {
        self.content.extend_from_slice(data);
    }

    pub fn write_varint(&mut self, mut value: u32) {
        loop {
            if (value & !0x7F) == 0 {
                self.write_u8(value as u8);
                return;
            }

            self.write_u8(((value & 0x7F) | 0x80).try_into().unwrap());

            value >>= 7;
        }
    }

    pub fn write_str(&mut self, s: &str) {
        self.write_varint(s.len() as u32);
        self.content.extend_from_slice(s.as_bytes());
    }
}
