mod arguments;

#[cfg(feature = "replace")]
pub mod replace;

use arguments::Arguments;
use clap::Parser;
use hsize::{Converter, Unit};

#[cfg(feature = "replace")]
use {arguments::MainSubcommand, regex::RegexBuilder, std::process::exit};

fn main() {
    let arguments = Arguments::parse();
    let converter = Converter {
        from_unit: Unit {
            is_binary: arguments.from_binary,
            scale: arguments.from_scale,
        },
        to_unit: Unit {
            is_binary: arguments.to_binary,
            scale: arguments.to_scale,
        },
    };
    let format_fn = |size: u128| -> String {
        converter.format_with_separator(size, arguments.precision, &arguments.separator)
    };

    match arguments.subcommand {
        #[cfg(feature = "replace")]
        Some(MainSubcommand::Replace {
            number_regex,
            multiline,
        }) => {
            if let Err(error) = replace::replace(
                &mut std::io::stdin().lines().map_while(Result::ok),
                &mut std::io::stdout(),
                &format_fn,
                match &RegexBuilder::new(&number_regex)
                    .multi_line(multiline)
                    .build()
                {
                    Ok(built_regex) => built_regex,
                    Err(error) => {
                        eprintln!("replace: {error}");
                        exit(1);
                    }
                },
            ) {
                eprintln!("write error: {error}");
                exit(1);
            };
        }
        _ => {
            if !arguments.sizes.is_empty() {
                for size in arguments.sizes {
                    println!("{}", format_fn(size));
                }
                return;
            }
            for (nr, line) in std::io::stdin()
                .lines()
                .map_while(Result::ok)
                .map(|line| line.trim().to_owned())
                .enumerate()
                .filter(|(_, line)| !line.is_empty())
            {
                if let Ok(number) = line.parse::<u128>() {
                    println!("{}", format_fn(number));
                } else {
                    eprintln!("invalid number found on line {}: {line}", nr + 1);
                };
            }
        }
    };
}
