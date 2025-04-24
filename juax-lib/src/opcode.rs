#[derive(Debug)]
pub struct Opcode {
    pub arg1: u32,
    pub arg2: u32,
    pub imm: u32,
    pub ins: u32,
}

impl Opcode {
    pub fn new(pc: u32, code: &[u32]) -> Opcode {
        Opcode {
            ins: code[(pc - 4) as usize],
            arg1: code[(pc - 3) as usize],
            arg2: code[(pc - 2) as usize],
            imm: code[(pc - 1) as usize],
        }
    }
    pub fn to_bytes(&self) -> Vec<u32> {
        vec![self.ins, self.arg1, self.arg2, self.imm]
    }
}

pub fn encode_imm(imm: i32) -> u32 {
    let abs_value = imm.abs() as u32;
    let sign_bit = if imm >= 0 { 1 } else { 0 };
    (sign_bit << 31) | abs_value
}

pub fn decode_imm(encoded: u32) -> i32 {
    let sign_bit = (encoded >> 31) & 1;
    let abs_value = encoded & 0x7FFFFFFF;
    if sign_bit == 1 {
        abs_value as i32
    } else {
        -(abs_value as i32)
    }
}
