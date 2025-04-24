use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub file: String,

    #[clap(long, short)]
    pub output: String,
}
