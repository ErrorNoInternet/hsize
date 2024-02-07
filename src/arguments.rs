use clap::{Parser, Subcommand};
use hsize::Scale;

#[cfg(any(feature = "completions", feature = "manpages"))]
use {
    clap::Command,
    std::io::{self},
};

#[cfg(feature = "completions")]
use clap_complete::{Generator, Shell};

#[cfg(feature = "manpages")]
use std::{
    path::{Path, PathBuf},
    process::exit,
};

#[cfg(feature = "replace")]
use clap::ValueHint;

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
    /// Use regex to search and replace numbers
    #[cfg(feature = "replace")]
    #[command(visible_aliases = ["r", "re"])]
    Replace {
        /// Regex to use for matching numbers
        #[arg(short, long, default_value = r"\d+")]
        regex: String,

        /// Enable multi-line regex searching
        #[arg(short = 'U', long)]
        multi_line: bool,

        /// Modify (search and replace) files in-place
        #[arg(short, long)]
        in_place: bool,

        #[arg(num_args = 1.., value_hint = ValueHint::FilePath)]
        files: Vec<String>,
    },

    /// Generate various CLI command features
    #[cfg(any(feature = "completions", feature = "manpages"))]
    #[command(visible_aliases = ["g", "gen"])]
    Generate {
        #[command(subcommand)]
        subcommand: GenerateSubcommand,
    },
}

#[cfg(any(feature = "completions", feature = "manpages"))]
#[derive(Debug, Subcommand)]
pub enum GenerateSubcommand {
    /// Shell completions
    #[cfg(feature = "completions")]
    #[command(visible_aliases = ["c", "comp"])]
    Completions {
        /// Output completion files for the specified shell
        #[arg(short, long)]
        shell: Shell,
    },

    /// Roff manpages
    #[cfg(feature = "manpages")]
    #[command(visible_aliases = ["m", "man"])]
    Manpages {
        /// Directory to save generated manpages
        #[arg(short, long, default_value = ".")]
        output_directory: PathBuf,
    },
}

#[cfg(feature = "completions")]
pub fn generate_completions<G: Generator>(generator: G, command: &mut Command) {
    clap_complete::generate(
        generator,
        command,
        command.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[cfg(feature = "manpages")]
pub fn generate_manpages(command: Command, output_directory: impl AsRef<Path>) {
    if let Err(error) = clap_mangen::generate_to(command, output_directory) {
        eprintln!("hsize: couldn't generate manpages: {error}");
        exit(1);
    };
}
