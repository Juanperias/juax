pub mod reg;
use reg::Reg;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cpu {
    code: Vec<u8>,
    regs: HashMap<Reg, i32>,
    pc: u32,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
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
    // ARG1 ARG2 INS MODE 
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.pc += 4;

        match self.code[(self.pc - 2) as usize] {
            0x10 => {
                let arg2 = self.code[(self.pc - 3) as usize];
                let arg1 = self.code[(self.pc - 4) as usize];

                println!("{} {}", arg1, arg2);
            }, // MOV
            u => {
                println!("{}", u)
            }, // 
        }

        Ok(())

    }
}
