use hsize::Converter;
use regex::RegexBuilder;
use std::io::Write;

pub fn replace<T: Iterator<Item = String>>(
    input: T,
    output: &mut dyn Write,

    converter: &Converter,
    number_regex_string: &str,
    multiline: bool,
) -> Result<(), String> {
    let number_regex = match RegexBuilder::new(number_regex_string)
        .multi_line(multiline)
        .build()
    {
        Ok(number_regex) => number_regex,
        Err(error) => return Err(format!("invalid regex specified: {error}")),
    };

    for line in input {
        let mut new_line = line.clone();
        for number in number_regex
            .find_iter(&line)
            .filter_map(|number_match| number_match.as_str().parse::<u128>().ok())
        {
            new_line = new_line.replace(&number.to_string(), converter.convert(number).as_str());
        }
        println!("{new_line}");
    }
}
