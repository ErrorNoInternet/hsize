mod arguments;
pub mod replace;

use arguments::{Arguments, MainSubcommand};
use clap::Parser;
use hsize::{Converter, Unit};
use regex::RegexBuilder;
use std::process::exit;

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
        if arguments.no_space {
            converter.format_with_separator(size, arguments.precision, "")
        } else {
            converter.format(size, arguments.precision)
        }
    };

    if let Some(MainSubcommand::Replace {
        number_regex,
        multiline,
    }) = arguments.subcommand
    {
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
        }
    } else {
        for size in arguments.sizes {
            println!("{}", format_fn(size));
        }
        if !atty::is(atty::Stream::Stdin) {
            for line in std::io::stdin().lines().map_while(Result::ok) {
                if let Ok(number) = line.trim().parse::<u128>() {
                    println!("{}", format_fn(number));
                } else {
                    eprintln!("invalid digit found in \"{line}\"");
                };
            }
        };
    }
}
