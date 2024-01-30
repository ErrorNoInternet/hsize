use clap::{Parser, Subcommand};
use hsize::Scale;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Arguments {
    #[arg(short, long, default_value_t = 2)]
    pub precision: usize,

    #[arg(short, long)]
    pub from_scale: Option<Scale>,

    #[arg(short = 'B', long)]
    pub from_binary: bool,

    #[arg(short, long)]
    pub to_scale: Option<Scale>,

    #[arg(short = 'b', long)]
    pub to_binary: bool,

    #[arg(num_args = 1..)]
    pub sizes: Vec<u128>,

    #[command(subcommand)]
    pub subcommand: Option<MainSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum MainSubcommand {
    Replace {
        #[arg(short = 'r', long, default_value = r"\d+")]
        number_regex: String,

        #[arg(short = 'U', long)]
        multiline: bool,
    },
}
