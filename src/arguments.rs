use clap::{Parser, Subcommand};
use hsize::Scale;

/// Convert file sizes to and from human-readable units
#[derive(Parser, Debug)]
#[command(version)]
pub struct Arguments {
    /// Number of decimal places to include in the converted number
    #[arg(short, long, default_value_t = 2)]
    pub precision: usize,

    /// Size scale of the specified (input) numbers
    #[arg(short, long, value_name = "SCALE")]
    pub from_scale: Option<Scale>,

    /// Whether or not the specified (input) numbers are powers of 2 (1K = 1024)
    #[arg(short = 'B', long)]
    pub from_binary: bool,

    /// Size scale of the converted numbers
    #[arg(short, long, value_name = "SCALE")]
    pub to_scale: Option<Scale>,

    /// Whether or not the converted numbers should be powers of 2 (1K = 1024)
    #[arg(short = 'b', long)]
    pub to_binary: bool,

    /// Whether or not to hide the space between the size and unit (1 KB -> 1KB)
    #[arg(short, long)]
    pub no_space: bool,

    #[arg(num_args = 1..)]
    pub sizes: Vec<u128>,

    #[command(subcommand)]
    pub subcommand: Option<MainSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum MainSubcommand {
    /// Use regex to search and replace numbers from stdin
    Replace {
        /// Regex pattern to use for matching numbers
        #[arg(short = 'r', long, default_value = r"\d+")]
        number_regex: String,

        /// Whether or not to enable multi-line mode
        #[arg(short = 'U', long)]
        multiline: bool,
    },
}
