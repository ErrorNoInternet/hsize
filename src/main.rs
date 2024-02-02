mod arguments;
mod cli;

#[cfg(feature = "replace")]
pub mod replace;

use arguments::Arguments;
use clap::Parser;
use hsize::{Converter, Unit};

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
    let formatter = |size: u128| -> String {
        converter.format_with_separator(size, arguments.precision, &arguments.separator)
    };

    cli::match_subcommand(&arguments, &formatter);
}
