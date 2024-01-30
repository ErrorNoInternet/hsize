mod arguments;

use arguments::Arguments;
use clap::Parser;
use hsize::Converter;

fn main() {
    let arguments = Arguments::parse();
    let converter = Converter {
        binary: arguments.binary,
        precision: arguments.precision,
        from_unit: arguments.from_unit,
        to_unit: arguments.to_unit,
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
