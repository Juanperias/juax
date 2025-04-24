pub mod cli;

use clap::Parser;
use cli::Cli;
use juax_lib::{Cpu, jli::JliFile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let jli = JliFile::read(cli.file)?;

    let mut cpu = Cpu::new(jli.content);

    cpu.run(cli.speed).unwrap();

    println!("{:?}", cpu.regs);

    Ok(())
}
