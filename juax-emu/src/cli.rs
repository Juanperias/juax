use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub file: String,

    #[clap(short, long)]
    pub speed: f64,
}
