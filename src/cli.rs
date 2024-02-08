use crate::arguments::Arguments;
use std::io::{self, Write};

#[cfg(any(feature = "replace", feature = "completions", feature = "manpages"))]
use crate::arguments::MainSubcommand;

#[cfg(any(feature = "completions", feature = "manpages"))]
use crate::arguments::GenerateSubcommand;

#[cfg(feature = "replace")]
use {
    crate::replace,
    regex::RegexBuilder,
    std::{
        fs,
        io::{BufRead, BufReader, BufWriter},
        process::exit,
        time,
    },
};

pub fn match_subcommand(arguments: &Arguments, formatter: &dyn Fn(u128) -> String) {
    match &arguments.subcommand {
        #[cfg(any(feature = "completions", feature = "manpages"))]
        Some(MainSubcommand::Generate { subcommand }) => match subcommand {
            #[cfg(feature = "completions")]
            GenerateSubcommand::Completions { shell } => {
                crate::arguments::generate_completions(shell.to_owned());
            }

            #[cfg(feature = "manpages")]
            GenerateSubcommand::Manpages { output_directory } => {
                if let Error(error) = crate::arguments::generate_manpages(output_directory) {
                    eprintln!("hsize: couldn't generate manpages: {error}");
                    exit(1);
                }
            }
        },

        #[cfg(feature = "replace")]
        Some(MainSubcommand::Replace {
            regex,
            multi_line,
            in_place,
            files,
        }) => {
            replace_subcommand(&formatter, regex, *multi_line, *in_place, files);
        }

        _ => {
            if !arguments.sizes.is_empty() {
                for size in &arguments.sizes {
                    let _ = io::stdout().write_all((formatter(*size) + "\n").as_bytes());
                }
                return;
            };

            for size in io::stdin()
                .lines()
                .map_while(Result::ok)
                .filter_map(|line| line.trim().parse::<u128>().ok())
            {
                let _ = io::stdout().write_all((formatter(size) + "\n").as_bytes());
            }
        }
    };
}

#[cfg(feature = "replace")]
fn replace_subcommand(
    formatter: &dyn Fn(u128) -> String,
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

    if files.is_empty() {
        for replaced_line in replace::replace(
            &mut io::stdin().lines().map_while(Result::ok),
            &built_regex,
            &formatter,
        ) {
            let _ = io::stdout().write_all((replaced_line + "\n").as_bytes());
        }
        return;
    }

    for file_path in files {
        let input_file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("hsize: couldn't open {file_path}: {error}");
                exit(2);
            }
        };
        let mut input_lines = BufReader::new(input_file).lines().map_while(Result::ok);

        if in_place {
            let temporary_file_path = file_path.to_owned() + ".hsize" + &random_string(8);
            let mut output_file_bufwriter = match fs::File::options()
                .write(true)
                .create(true)
                .open(&temporary_file_path)
            {
                Ok(file) => BufWriter::new(file),
                Err(error) => {
                    eprintln!("hsize: couldn't open temporary file {temporary_file_path}: {error}");
                    exit(3);
                }
            };

            for replaced_line in replace::replace(&mut input_lines, &built_regex, &formatter) {
                if let Err(error) =
                    output_file_bufwriter.write_all((replaced_line + "\n").as_bytes())
                {
                    eprintln!(
                        "hsize: couldn't write to temporary file {temporary_file_path}: {error}"
                    );
                    exit(4);
                }
            }

            if let Err(error) = fs::rename(&temporary_file_path, file_path) {
                eprintln!(
                        "hsize: couldn't rename temporary file {temporary_file_path} to {file_path}: {error}"
                    );
                exit(5);
            };
        } else {
            for replaced_line in replace::replace(&mut input_lines, &built_regex, &formatter) {
                let _ = io::stdout().write_all((replaced_line + "\n").as_bytes());
            }
        };
    }
}

#[cfg(feature = "replace")]
fn random_string(length: usize) -> String {
    let mut rng = oorandom::Rand64::new(
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
    );
    let mut string = String::with_capacity(length);
    for _ in 0..length {
        string.push(
            char::from_u32(u32::try_from(rng.rand_range(65..91)).unwrap_or(65)).unwrap_or('A'),
        );
    }
    string
}
