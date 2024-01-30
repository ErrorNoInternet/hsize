mod arguments;

use arguments::Arguments;
use clap::Parser;
use hsize::Converter;

fn main() {
    let arguments = Arguments::parse();
    let converter = Converter {
        binary: arguments.binary,
        precision: arguments.precision,
        from: arguments.from,
        to: arguments.to,
    };

    for size in arguments.sizes {
        println!("{}", converter.humanize(size))
    }
    if !atty::is(atty::Stream::Stdin) {
        for line in std::io::stdin().lines().flatten() {
            if let Ok(number) = line.trim().parse::<u128>() {
                println!("{}", converter.humanize(number));
            } else {
                eprintln!("invalid digit found in \"{line}\"");
            };
        }
    };
}
