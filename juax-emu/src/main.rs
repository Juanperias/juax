use juax_lib::Cpu;

fn main() {
    let mut cpu = Cpu::new(vec![0x1, 0x2, 0x10, 0x0]);

    cpu.run().unwrap();
    
    println!("{:?}", cpu.regs);
}
