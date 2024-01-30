use clap::{Parser, Subcommand};
use hsize::Scale;
use regex::Regex;

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
        #[arg(short, long, default_value = r"[0-9]+")]
        number_regex: Regex,

        #[arg(
            short,
            long,
            default_value = r"((K|k|M|m|G|g|T|t|P|p|E|e|Z|z|Y|y)?(B|b))"
        )]
        unit_regex: Regex,
    },
}
