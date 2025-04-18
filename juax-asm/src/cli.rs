use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub file: String,
    pub output: String,
}
