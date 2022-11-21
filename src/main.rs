use clap::{value_parser, Arg, ArgAction, Command};
use num_traits::FromPrimitive;
#[macro_use]
extern crate num_derive;

#[derive(Debug, PartialEq, FromPrimitive)]
enum SizeUnit {
    B,
    K,
    M,
    G,
    T,
    P,
    E,
    Z,
    Y,
}

fn display_unit(unit: &SizeUnit, binary: bool) -> String {
    let mut output = format!("{:?}", unit);
    if unit != &SizeUnit::B {
        output.push('B');
        if binary {
            output.insert(1, 'i')
        }
    }
    output
}

fn main() {
    let command = Command::new("hsize")
        .arg(
            Arg::new("binary")
                .long("binary")
                .short('b')
                .action(ArgAction::SetTrue)
                .help("Display sizes in binary (powers of 1024)"),
        )
        .arg(
            Arg::new("precision")
                .long("precision")
                .short('p')
                .value_parser(value_parser!(usize))
                .help("The amount of decimal places to display"),
        )
        .arg(
            Arg::new("sizes")
                .num_args(1..)
                .value_parser(value_parser!(u128))
                .help("The sizes in bytes, to be converted to their appropriate units"),
        );
    let matches = command.get_matches();
    let argument_binary = matches.get_one("binary").unwrap_or(&false);
    let argument_precision = matches.get_one("precision").unwrap_or(&2);
    let mut sizes = Vec::new();
    match matches.get_many::<u128>("sizes") {
        Some(matches) => {
            for size_match in matches {
                sizes.push(size_match.to_owned())
            }
        }
        None => (),
    }

    if !atty::is(atty::Stream::Stdin) {
        for line in std::io::stdin().lines() {
            match line {
                Ok(line) => match line.trim().parse::<u128>() {
                    Ok(number) => sizes.push(number),
                    Err(_) => println!("invalid digit found in \"{}\"", line),
                },
                Err(_) => (),
            }
        }
    }

    for size in sizes {
        let divisor = if *argument_binary { 1024 } else { 1000 };
        let mut new_size = size as f64;
        let mut unit = SizeUnit::B;
        while new_size >= divisor as f64 {
            if let Some(new_unit) = SizeUnit::from_u32(unit as u32 + 1) {
                unit = new_unit
            } else {
                break;
            }
            new_size = new_size / divisor as f64;
        }
        println!(
            "{:.argument_precision$} {}",
            new_size,
            display_unit(&unit, *argument_binary)
        );
    }
}
