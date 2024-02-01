mod arguments;

#[cfg(feature = "replace")]
pub mod replace;

use arguments::Arguments;
use clap::Parser;
use hsize::{Converter, Unit};
use std::io;

#[cfg(feature = "replace")]
use {
    arguments::MainSubcommand,
    regex::RegexBuilder,
    std::{
        fs,
        io::{BufRead, BufReader, Write},
        process::exit,
    },
};

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
            in_place,
            files,
        }) => {
            subcommand_replace(&format_fn, &number_regex, multiline, in_place, &files);
        }

        _ => {
            if !arguments.sizes.is_empty() {
                for size in arguments.sizes {
                    println!("{}", format_fn(size));
                }
                return;
            }
            for (nr, line) in io::stdin()
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

#[cfg(feature = "replace")]
fn subcommand_replace(
    format_fn: &dyn Fn(u128) -> String,
    number_regex: &str,
    multiline: bool,
    in_place: bool,
    files: &Vec<String>,
) {
    let built_regex = match RegexBuilder::new(number_regex)
        .multi_line(multiline)
        .build()
    {
        Ok(built_regex) => built_regex,
        Err(error) => {
            eprintln!("replace: {error}");
            exit(1);
        }
    };
    let replace_fn = |input: &mut dyn Iterator<Item = String>, output: &mut dyn Write| {
        if let Err(error) = replace::replace(input, output, &format_fn, &built_regex) {
            eprintln!("write: {error}");
            exit(1);
        };
    };

    if files.is_empty() {
        replace_fn(
            &mut io::stdin().lines().map_while(Result::ok),
            &mut io::stdout(),
        );
    } else {
        for file_path in files {
            let input_file = match fs::File::open(file_path) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("open: {file_path}: {error}");
                    continue;
                }
            };
            if in_place {
                let temporary_file_path = file_path.clone() + ".tmp";
                let mut output_file = match fs::File::options()
                    .write(true)
                    .create(true)
                    .open(&temporary_file_path)
                {
                    Ok(file) => file,
                    Err(error) => {
                        eprintln!("open: {temporary_file_path}: {error}");
                        continue;
                    }
                };
                replace_fn(
                    &mut BufReader::new(input_file).lines().map_while(Result::ok),
                    &mut output_file,
                );
                if let Err(error) = fs::rename(temporary_file_path, file_path) {
                    eprintln!("rename: {file_path}: {error}");
                };
            } else {
                replace_fn(
                    &mut BufReader::new(input_file).lines().map_while(Result::ok),
                    &mut io::stdout(),
                );
            };
        }
    }
}
