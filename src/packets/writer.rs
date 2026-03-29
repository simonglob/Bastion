pub struct Writer {
    content: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self { content: vec![] }
    }

    pub fn get_content(&self) -> &Vec<u8> {
        &self.content
    }

    pub fn write_u8(&mut self, i: u8) {
        self.content.push(i);
    }

    pub fn write_u16(&mut self, i: u16) {
        self.content.extend(i.to_be_bytes());
    }

    pub fn write_u32(&mut self, i: u32) {
        self.content.extend(i.to_be_bytes());
    }

    pub fn write_i64(&mut self, i: i64) {
        self.content.extend(i.to_be_bytes());
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

    pub fn write_all(&mut self, arr: &Vec<u8>) {
        self.content.extend_from_slice(arr);
    }
}
