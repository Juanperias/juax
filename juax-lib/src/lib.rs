pub mod reg;
pub mod opcode;
pub mod ins;

use ins::mov::process_mov;
use reg::Reg;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cpu {
    pub code: Vec<i32>,
    pub regs: HashMap<Reg, i32>,
    pub pc: u32,
}

impl Cpu {
    pub fn new(code: Vec<i32>) -> Self {
        Cpu {
            code,
            regs: HashMap::from([
                (Reg::A0, 0),
                (Reg::A1, 0),
            ]),
            pc: 0,
        }
    }
    pub fn get_reg(&self, reg: Reg) -> i32 {
        *self.regs.get(&reg).unwrap()
    }
    pub fn set_reg(&mut self, reg: Reg, value: i32) {
         *self.regs.get_mut(&reg).unwrap() = value;
    }
    // INS ARG2 ARG1 IMM
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.pc += 4;
        let opcode = opcode::Opcode::new(self.pc, &self.code);
        
        match opcode.ins {
            0x4 => {
                process_mov(self, &opcode)?;
            },
            0x2 => {
                
            },
            u => {
                println!("{}", u)
            },
        }

        Ok(())

    }
}
