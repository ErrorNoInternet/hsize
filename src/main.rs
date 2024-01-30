mod arguments;
pub mod replace;

use arguments::{Arguments, MainSubcommand};
use clap::Parser;
use hsize::{Converter, Unit};
use std::process::exit;

fn main() {
    let arguments = Arguments::parse();
    let converter = Converter {
        precision: arguments.precision,
        from_unit: Unit {
            binary: arguments.from_binary,
            scale: arguments.from_scale,
        },
        to_unit: Unit {
            binary: arguments.to_binary,
            scale: arguments.to_scale,
        },
    };

    if let Some(MainSubcommand::Replace {
        number_regex,
        multiline,
    }) = arguments.subcommand
    {
        if let Err(error) = replace::replace(
            &mut std::io::stdin().lines().map_while(Result::ok),
            &mut std::io::stdout(),
            &converter,
            &number_regex,
            multiline,
        ) {
            match error {
                replace::Error::Regex(error) => eprintln!("replace: {error}"),
                replace::Error::Write(error) => eprintln!("write error: {error}"),
            }
            exit(1);
        }
    } else {
        for size in arguments.sizes {
            println!("{}", converter.convert(size));
        }
        if !atty::is(atty::Stream::Stdin) {
            for line in std::io::stdin().lines().map_while(Result::ok) {
                if let Ok(number) = line.trim().parse::<u128>() {
                    println!("{}", converter.convert(number));
                } else {
                    eprintln!("invalid digit found in \"{line}\"");
                };
            }
        };
    }
}
