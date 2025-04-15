use clap::{Parser, Subcommand};
use hsize::Scale;

#[cfg(any(feature = "replace", feature = "manpages"))]
use clap::ValueHint;

#[cfg(any(feature = "completions", feature = "manpages"))]
use {clap::CommandFactory, std::io};

#[cfg(feature = "completions")]
use clap_complete::{Generator, Shell};

#[cfg(feature = "manpages")]
use std::path::{Path, PathBuf};

/// Convert file sizes to and from human-readable units
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[command(version)]
pub struct Arguments {
    /// Number of decimal places to display in the converted number
    #[arg(short, long, env = "HSIZE_PRECISION", default_value_t = 1)]
    pub precision: usize,

    /// Size scale of the given numbers
    #[arg(short, long, value_name = "SCALE", env = "HSIZE_FROM_SCALE")]
    pub from_scale: Option<Scale>,

    /// Given numbers are powers of 2 (1K = 1024)
    #[arg(short = 'B', long, env = "HSIZE_FROM_BINARY")]
    pub from_binary: bool,

    /// Size scale of the converted numbers
    #[arg(short, long, value_name = "SCALE", env = "HSIZE_TO_SCALE")]
    pub to_scale: Option<Scale>,

    /// Converted numbers should be powers of 2 (1K = 1024)
    #[arg(short = 'b', long, env = "HSIZE_TO_BINARY")]
    pub to_binary: bool,

    /// Numbers should be displayed in scientific notation
    #[arg(short = 'e', long, env = "HSIZE_SCIENTIFIC_NOTATION")]
    pub scientific_notation: bool,

    /// Characters to put between the number and unit
    #[arg(short, long, env = "HSIZE_SEPARATOR", default_value = " ")]
    pub separator: String,

    /// Remove the 'B' from the end of the unit (MB -> M)
    #[arg(short, long, env = "HSIZE_NO_B_SUFFIX")]
    pub no_b_suffix: bool,

    /// Skip converting numbers if they'll end up being longer than the original
    #[arg(short = 'S', long, env = "HSIZE_SKIP_SHORT_NUMBERS")]
    pub skip_short_numbers: bool,

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
        #[arg(short, long, env = "HSIZE_REGEX", default_value = r"\d+")]
        regex: String,

        /// Enable multi-line regex searching
        #[arg(short = 'U', long)]
        multi_line: bool,

        /// Don't align converted numbers to the right
        #[arg(short = 'L', long, env = "HSIZE_LEFT_ALIGN")]
        left_align: bool,

        /// Modify (search and replace) files in-place
        #[arg(short, long)]
        in_place: bool,

        #[arg(num_args = 1.., value_hint = ValueHint::FilePath)]
        files: Vec<String>,
    },

    /// Generate various shell command files
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
        /// Output completion files for the given shell
        #[arg(short, long)]
        shell: Shell,
    },

    /// Roff manpages
    #[cfg(feature = "manpages")]
    #[command(visible_aliases = ["m", "man"])]
    Manpages {
        /// Directory to save generated manpages
        #[arg(short, long, default_value = ".", value_hint = ValueHint::DirPath)]
        output_directory: PathBuf,
    },
}

#[cfg(feature = "completions")]
pub fn generate_completions<G: Generator>(generator: G) {
    let command = Arguments::command();
    clap_complete::generate(
        generator,
        &mut command.clone(),
        command.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[cfg(feature = "manpages")]
pub fn generate_manpages(output_directory: impl AsRef<Path>) -> Result<(), io::Error> {
    clap_mangen::generate_to(Arguments::command(), output_directory)
}

#[cfg(test)]
mod test {
    #[cfg(feature = "manpages")]
    #[test]
    fn generate_manpages() {
        super::generate_manpages(".").unwrap();
    }
}
