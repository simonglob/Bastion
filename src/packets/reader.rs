use bytes::{Buf, Bytes};
use uuid::Uuid;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub trait MinecraftProtocol {
    fn get_var_int(&mut self) -> i32;
    fn get_string(&mut self) -> String;
    fn get_bytearray(&mut self) -> Vec<u8>;
    fn get_uuid(&mut self) -> Uuid;
}

impl MinecraftProtocol for Bytes {
    fn get_var_int(&mut self) -> i32 {
        let mut value = 0;
        let mut shift = 0;

        loop {
            let current_byte = self.get_u8();
            value |= ((current_byte & SEGMENT_BITS) as i32) << shift;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            shift += 7;

            if shift >= 32 {
                panic!("VarInt is too big")
            }
        }

        value
    }

    fn get_string(&mut self) -> String {
        let length = self.get_var_int() as usize;
        let mut string_bytes = vec![0u8; length];
        self.copy_to_slice(&mut string_bytes);

        String::from_utf8(string_bytes).expect("Invalid UTF-8")
    }

    fn get_bytearray(&mut self) -> Vec<u8> {
        let length = self.get_var_int() as usize;
        let mut ret = vec![0u8; length];
        self.copy_to_slice(&mut ret);
        ret
    }

    fn get_uuid(&mut self) -> Uuid {
        Uuid::from_u128(self.get_u128())
    }
}
