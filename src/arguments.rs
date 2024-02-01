use clap::{Parser, Subcommand};
use hsize::Scale;

/// Convert file sizes to and from human-readable units
#[derive(Parser, Debug)]
#[command(version)]
pub struct Arguments {
    /// Number of decimal places to include in the converted number
    #[arg(short, long, env = "HSIZE_PRECISION", default_value_t = 2)]
    pub precision: usize,

    /// Size scale of the specified (input) numbers
    #[arg(short, long, value_name = "SCALE")]
    pub from_scale: Option<Scale>,

    /// Whether or not the specified (input) numbers are powers of 2 (1K = 1024)
    #[arg(short = 'B', long, env = "HSIZE_FROM_BINARY")]
    pub from_binary: bool,

    /// Size scale of the converted numbers
    #[arg(short, long, value_name = "SCALE")]
    pub to_scale: Option<Scale>,

    /// Whether or not the converted numbers should be powers of 2 (1K = 1024)
    #[arg(short = 'b', long, env = "HSIZE_TO_BINARY")]
    pub to_binary: bool,

    /// Character(s) to put between the number and unit
    #[arg(short, long, env = "HSIZE_SEPARATOR", default_value = " ")]
    pub separator: String,

    #[arg(num_args = 1..)]
    pub sizes: Vec<u128>,

    #[command(subcommand)]
    pub subcommand: Option<MainSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum MainSubcommand {
    /// Use regex to search and replace numbers from stdin
    #[cfg(feature = "replace")]
    #[command(visible_aliases = ["r", "re"])]
    Replace {
        /// Regex to use for matching numbers
        #[arg(short = 'r', long, default_value = r"\d+")]
        number_regex: String,

        /// Enable multi-line regex searching
        #[arg(short = 'U', long)]
        multiline: bool,
    },
}
