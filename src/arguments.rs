use clap::Parser;
use hsize::SizeUnit;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Arguments {
    #[arg(short, long)]
    pub binary: bool,

    #[arg(short, long, default_value_t = 2)]
    pub precision: usize,

    #[arg(short, long)]
    pub from: Option<SizeUnit>,

    #[arg(short, long)]
    pub to: Option<SizeUnit>,

    #[arg(num_args = 1..)]
    pub sizes: Vec<u128>,
}
