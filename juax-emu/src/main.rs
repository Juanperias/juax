use juax_lib::{Cpu, opcode::{encode_imm, decode_imm}, reg::Reg};

fn main() {
    let mut cpu = Cpu::new(vec![
        0x2,
        0x2,
        0x0,
        10,

        0x4, 
        0x1, 
        0x2, 
        0x00,
        
        0x2,
        0x1,
        0x0,
        encode_imm(-41),
    ]);

    cpu.run().unwrap();

    println!("{}", decode_imm(*cpu.regs.get(&Reg::A0).unwrap()));
}
