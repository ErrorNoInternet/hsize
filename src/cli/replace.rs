use crate::replace;
use regex::RegexBuilder;
use std::{
    fs,
    io::{self, BufRead, BufReader, BufWriter, Write},
    iter::repeat_with,
    process::exit,
};

pub fn replace(
    formatter: &dyn Fn(u128) -> String,
    regex: &str,
    multiline: bool,
    left_align: bool,
    in_place: bool,
    files: &Vec<String>,
) {
    let built_regex = match RegexBuilder::new(regex).multi_line(multiline).build() {
        Ok(regex) => regex,
        Err(error) => {
            eprintln!("hsize replace: {error}");
            exit(3);
        }
    };

    if files.is_empty() {
        for replaced_line in replace::replace(
            &mut io::stdin().lines().map_while(Result::ok),
            &built_regex,
            &formatter,
            left_align,
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
                exit(4);
            }
        };
        let mut input_lines = BufReader::new(input_file).lines().map_while(Result::ok);

        if in_place {
            let temporary_file_path = file_path.to_owned()
                + ".hsize"
                + &repeat_with(fastrand::alphanumeric)
                    .take(8)
                    .collect::<String>();
            let mut output_file_bufwriter = match fs::File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&temporary_file_path)
            {
                Ok(file) => BufWriter::new(file),
                Err(error) => {
                    eprintln!("hsize: couldn't open temporary file {temporary_file_path}: {error}");
                    exit(5);
                }
            };

            for replaced_line in
                replace::replace(&mut input_lines, &built_regex, &formatter, left_align)
            {
                if let Err(error) =
                    output_file_bufwriter.write_all((replaced_line + "\n").as_bytes())
                {
                    eprintln!(
                        "hsize: couldn't write to temporary file {temporary_file_path}: {error}"
                    );
                    exit(6);
                }
            }

            if let Err(error) = fs::rename(&temporary_file_path, file_path) {
                eprintln!(
                    "hsize: couldn't rename temporary file {temporary_file_path} to {file_path}: {error}"
                );
                exit(7);
            }
        } else {
            for replaced_line in
                replace::replace(&mut input_lines, &built_regex, &formatter, left_align)
            {
                let _ = io::stdout().write_all((replaced_line + "\n").as_bytes());
            }
        }
    }
}
