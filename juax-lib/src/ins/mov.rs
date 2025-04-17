use crate::reg::{RegError, Reg};
use crate::Cpu;
use crate::opcode::Opcode;


pub fn process_mov(cpu: &mut Cpu, opcode: &Opcode) -> Result<(), RegError> { 
    let arg2 = Reg::try_from(opcode.arg2)?;
    let arg1 = Reg::try_from(opcode.arg1)?;

    cpu.set_reg(arg1, cpu.get_reg(arg2));

    Ok(())
}
