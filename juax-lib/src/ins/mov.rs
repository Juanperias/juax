use thiserror::Error;
use crate::reg::{RegError, Reg};
use crate::Cpu;

#[derive(Error, Debug)]
pub enum MovError {
    #[error("Reg Error: {0}")]
    RegError(#[from] RegError),
}

pub fn process_mov(cpu: &mut Cpu) -> Result<(), MovError> { 
    let arg2 = Reg::try_from(cpu.code[(cpu.pc - 3) as usize])?;
    let arg1 = Reg::try_from(cpu.code[(cpu.pc - 4) as usize])?;

    cpu.set_reg(arg1, cpu.get_reg(arg2));

    Ok(())
}
