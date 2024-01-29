use clap::Parser;
use hsize::{display_size, SizeUnit};

/// Convert file sizes from bytes to human-readable units
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

fn main() {
    let arguments = Arguments::parse();
    for size in arguments.sizes {
        println!(
            "{}",
            display_size(size, arguments.binary, arguments.unit, arguments.precision)
        )
    }

    if !atty::is(atty::Stream::Stdin) {
        for line in std::io::stdin().lines().flatten() {
            match line.trim().parse::<u128>() {
                Ok(number) => println!(
                    "{}",
                    display_size(
                        number,
                        arguments.binary,
                        arguments.unit,
                        arguments.precision
                    )
                ),
                Err(_) => eprintln!("invalid digit found in \"{line}\""),
            };
        }
    }
}
