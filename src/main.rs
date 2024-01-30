mod arguments;
mod replace;

use arguments::{Arguments, MainSubcommand};
use clap::Parser;
use hsize::{Converter, Unit};

fn main() {
    let arguments = Arguments::parse();

    if let Some(MainSubcommand::Replace {
        ref number_regex,
        ref unit_regex,
    }) = arguments.subcommand
    {
        replace::replace(number_regex, unit_regex);
    } else {
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
