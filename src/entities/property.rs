use crate::packets::{codec::Encode, writer::Writer};

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl Encode for Property {
    fn encode(&self, writer: &mut Writer) {
        writer.write_str(&self.name);
        writer.write_str(&self.value);

        match &self.signature {
            Some(sig) => {
                writer.write_u8(0x01);
                writer.write_str(sig);
            }
            None => {
                writer.write_u8(0x00);
            }
        }
    }
}

#[derive(Debug)]
pub struct Properties {
    pub entries: Vec<Property>,
}

impl Properties {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }
}

impl Encode for Properties {
    fn encode(&self, writer: &mut Writer) {
        writer.write_varint(self.entries.len() as i32);
        self.entries.iter().for_each(|f| {
            f.encode(writer);
        });
    }
}
