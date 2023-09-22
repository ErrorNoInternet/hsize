use clap::{Parser, ValueEnum};
use num_traits::FromPrimitive;
#[macro_use]
extern crate num_derive;

/// Convert file sizes in bytes to human-readable units
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Display sizes in binary (powers of 1024)
    #[arg(short, long)]
    binary: bool,

    /// The amount of decimal places to display
    #[arg(short, long, default_value_t = 2)]
    precision: usize,

    /// The unit to display sizes in
    #[arg(short, long)]
    unit: Option<SizeUnit>,

    /// The file sizes in bytes, to be converted to their appropriate units
    #[arg(num_args = 1..)]
    sizes: Vec<u128>,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ValueEnum)]
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

fn display_unit(binary: bool, unit: SizeUnit) -> String {
    let mut output = format!("{:?}", unit);
    if unit != SizeUnit::B {
        output.push('B');
        if binary {
            output.insert(1, 'i')
        }
    }
    output
}

fn display_size(size: u128, binary: bool, unit: Option<SizeUnit>, precision: usize) -> String {
    let divisor: u128 = if binary { 1024 } else { 1000 };
    let mut current_size = size as f64;
    let mut current_unit = SizeUnit::B;

    if unit.is_none() {
        while current_size >= divisor as f64 {
            if let Some(new_unit) = SizeUnit::from_u32(current_unit as u32 + 1) {
                current_unit = new_unit
            } else {
                break;
            }
            current_size = current_size / divisor as f64;
        }
    } else {
        current_unit = unit.unwrap();
        current_size = current_size / divisor.pow(current_unit as u32) as f64
    }
    format!(
        "{:.precision$} {}",
        current_size,
        display_unit(binary, current_unit)
    )
}

fn main() {
    let arguments = Arguments::parse();
    let mut sizes = arguments.sizes;

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
        println!(
            "{}",
            display_size(size, arguments.binary, arguments.unit, arguments.precision)
        )
    }
}
