mod arguments;

#[cfg(feature = "replace")]
pub mod replace;

use arguments::Arguments;
use clap::Parser;
use hsize::{Converter, Unit};
use std::io::{self, BufWriter, Write};

#[cfg(feature = "replace")]
use {
    arguments::MainSubcommand,
    regex::RegexBuilder,
    std::{
        fs,
        io::{BufRead, BufReader, StdoutLock},
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

    let mut stdout_bufwriter = BufWriter::new(io::stdout().lock());
    let format = |size: u128| -> String {
        converter.format_with_separator(size, arguments.precision, &arguments.separator)
    };

    match arguments.subcommand {
        #[cfg(feature = "replace")]
        Some(MainSubcommand::Replace {
            regex,
            multi_line,
            in_place,
            files,
        }) => {
            subcommand_replace(
                &mut stdout_bufwriter,
                &format,
                &regex,
                multi_line,
                in_place,
                &files,
            );
        }

        _ => {
            if !arguments.sizes.is_empty() {
                for size in arguments.sizes {
                    let _ = writeln!(stdout_bufwriter, "{}", format(size));
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
                if let Ok(size) = line.parse::<u128>() {
                    let _ = writeln!(stdout_bufwriter, "{}", format(size));
                } else {
                    eprintln!("hsize: invalid number on line {}: {line}", nr + 1);
                };
            }
        }
    };
}

#[cfg(feature = "replace")]
fn subcommand_replace(
    stdout_bufwriter: &mut BufWriter<StdoutLock>,
    format: &dyn Fn(u128) -> String,
    regex: &str,
    multiline: bool,
    in_place: bool,
    files: &Vec<String>,
) {
    let built_regex = match RegexBuilder::new(regex).multi_line(multiline).build() {
        Ok(built_regex) => built_regex,
        Err(error) => {
            eprintln!("hsize replace: {error}");
            exit(1);
        }
    };
    let replace = |input: &mut dyn Iterator<Item = String>, output: &mut dyn Write| {
        for line in replace::replace(input, &format, &built_regex) {
            if let Err(error) = writeln!(output, "{line}") {
                eprintln!("hsize replace: write error: {error}");
                exit(2);
            }
        }
    };

    if files.is_empty() {
        replace(
            &mut io::stdin().lines().map_while(Result::ok),
            stdout_bufwriter,
        );
    } else {
        for file_path in files {
            let input_file = match fs::File::open(file_path) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("hsize replace: open error: {file_path}: {error}");
                    continue;
                }
            };
            let mut input = BufReader::new(input_file).lines().map_while(Result::ok);
            if in_place {
                let temporary_file_path = file_path.clone() + ".tmp";
                let mut output_file = match fs::File::options()
                    .write(true)
                    .create(true)
                    .open(&temporary_file_path)
                {
                    Ok(file) => file,
                    Err(error) => {
                        eprintln!("hsize replace: create error: {temporary_file_path}: {error}");
                        continue;
                    }
                };
                replace(&mut input, &mut output_file);
                if let Err(error) = fs::rename(&temporary_file_path, file_path) {
                    eprintln!("hsize replace: rename error: {temporary_file_path} to {file_path}: {error}");
                };
            } else {
                replace(&mut input, stdout_bufwriter);
            };
        }
    }
}
