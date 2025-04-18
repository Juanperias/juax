use crate::Cpu;
use crate::opcode::Opcode;
use crate::reg::{Reg, RegError};

pub fn process_load(cpu: &mut Cpu, opcode: &Opcode) -> Result<(), RegError> {
    let dist = Reg::try_from(opcode.arg1)?;

    cpu.set_reg(dist, opcode.imm);

    Ok(())
}
