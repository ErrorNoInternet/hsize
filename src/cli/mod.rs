#[cfg(feature = "replace")]
mod replace;

use crate::arguments::Arguments;
use std::io::{self, Write};

#[cfg(any(feature = "replace", feature = "completions", feature = "manpages"))]
use crate::arguments::MainSubcommand;

#[cfg(any(feature = "completions", feature = "manpages"))]
use crate::arguments::GenerateSubcommand;

pub fn main(arguments: &Arguments, formatter: &dyn Fn(u128) -> String) {
    match &arguments.subcommand {
        #[cfg(any(feature = "completions", feature = "manpages"))]
        Some(MainSubcommand::Generate { subcommand }) => match subcommand {
            #[cfg(feature = "completions")]
            GenerateSubcommand::Completions { shell } => {
                crate::arguments::generate_completions(shell.to_owned());
            }

            #[cfg(feature = "manpages")]
            GenerateSubcommand::Manpages { output_directory } => {
                if let Err(error) = crate::arguments::generate_manpages(output_directory) {
                    eprintln!("hsize: couldn't generate manpages: {error}");
                    std::process::exit(1);
                }
            }
        },

        #[cfg(feature = "replace")]
        Some(MainSubcommand::Replace {
            regex,
            multi_line,
            left_align,
            in_place,
            files,
        }) => {
            replace::replace(
                &formatter,
                regex,
                *multi_line,
                *left_align,
                *in_place,
                files,
            );
        }

        _ => {
            if !arguments.sizes.is_empty() {
                for size in &arguments.sizes {
                    let _ = io::stdout().write_all((formatter(*size) + "\n").as_bytes());
                }
                return;
            }

            for line in io::stdin().lines().map_while(Result::ok) {
                if let Ok(size) = line.trim().parse::<u128>() {
                    let _ = io::stdout().write_all((formatter(size) + "\n").as_bytes());
                } else {
                    eprintln!("hsize: invalid number: {line:?}");
                }
            }
        }
    }
}
