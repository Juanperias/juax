use juax_lib::Cpu;

fn main() {
    let mut cpu = Cpu::new(vec![0x4, 0x1, 0x2, 0x00]);

    cpu.run().unwrap();
    
    println!("{:?}", cpu.regs);
}
