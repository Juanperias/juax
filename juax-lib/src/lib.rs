pub mod ins;
pub mod jli;
pub mod opcode;
pub mod reg;

use ins::{load::process_load, mov::process_mov};
use reg::Reg;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Cpu {
    pub code: Vec<u32>,
    pub regs: HashMap<Reg, u32>,
    pub pc: u32,
}

impl Cpu {
    pub fn new(code: Vec<u32>) -> Self {
        Cpu {
            code,
            regs: HashMap::from([(Reg::A0, 0), (Reg::A1, 0)]),
            pc: 0,
        }
    }
    pub fn get_reg(&self, reg: Reg) -> u32 {
        *self.regs.get(&reg).unwrap()
    }
    pub fn set_reg(&mut self, reg: Reg, value: u32) {
        *self.regs.get_mut(&reg).unwrap() = value;
    }
    // INS ARG2 ARG1 IMM
    pub fn run(&mut self, frequency: f64) -> Result<(), Box<dyn std::error::Error>> {
        let target_duration = Duration::from_secs_f64(1.0 / frequency);

        while (self.pc as usize) < self.code.len() {
            let start_time = Instant::now();

            self.pc += 4;
            let opcode = opcode::Opcode::new(self.pc, &self.code);

            match opcode.ins {
                0x4 => {
                    process_mov(self, &opcode)?;
                }
                0x2 => {
                    process_load(self, &opcode)?;
                }
                u => {
                    println!("{}", u);
                }
            }

            let elapsed_time = start_time.elapsed();
            if elapsed_time < target_duration {
                std::thread::sleep(target_duration - elapsed_time);
            }
        }

        Ok(())
    }
}
