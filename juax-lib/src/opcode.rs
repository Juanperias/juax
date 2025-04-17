#[derive(Debug)]
pub struct Opcode {
    pub arg1: i32,
    pub arg2: i32,
    pub imm: i32,
    pub ins: i32
}

impl Opcode {
    pub fn new(pc: u32, code: &[i32]) -> Opcode {
        Opcode {
            ins: code[(pc - 4) as usize],
            arg1: code[(pc - 3) as usize],
            arg2: code[(pc - 2) as usize],
            imm: code[(pc - 1) as usize]
        }
    }
}
