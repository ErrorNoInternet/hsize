use crate::{Converter, Unit};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Options<'a> {
    pub precision: usize,
    pub separator: &'a str,
    pub scientific_notation: bool,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            precision: 1,
            separator: " ",
            scientific_notation: false,
        }
    }
}

impl Converter {
    pub fn format(&self, size: u128, precision: usize) -> String {
        self.format_with_options(
            size,
            &Options {
                precision,
                ..Options::default()
            },
        )
    }

    pub fn format_with_options(&self, size: u128, options: &Options) -> String {
        let (new_size, scale) = self.convert(size);
        let unit = Unit {
            is_binary: self.to_unit.is_binary,
            scale: Some(scale),
        }
        .to_string();
        if options.scientific_notation {
            format!(
                "{new_size:.*e}{}{unit}",
                options.precision, options.separator,
            )
        } else {
            format!(
                "{new_size:.*}{}{unit}",
                options.precision, options.separator,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{format::Options, Converter, Scale, Unit};

    #[test]
    fn basic() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };

        assert_eq!(converter.format(0, 0), "0 B");
        assert_eq!(converter.format(123, 0), "123 B");
        assert_eq!(converter.format(5555, 0), "5 KiB");
        assert_eq!(converter.format(1_048_576, 0), "1 MiB");
        assert_eq!(converter.format(1024 * 1024 * 1024, 0), "1 GiB");
    }

    #[test]
    fn big() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };

        assert_eq!(converter.format(10_101_010_101_010_101, 5), "10.10101 PB");
        assert_eq!(converter.format(123_456_789, 5), "123.45679 MB");
        assert_eq!(converter.format(1_111_111_111_111_111_111, 5), "1.11111 EB");
        assert_eq!(converter.format(999_999, 5), "999.99900 KB");
        assert_eq!(
            converter.format(5_555_555_555_555_555_555_555_555_555_555, 5),
            "5555555.55556 YB"
        );
        assert_eq!(
            converter.format_with_options(
                1000_u128.pow(10),
                &Options {
                    precision: 0,
                    separator: " ",
                    scientific_notation: true,
                }
            ),
            "1e6 YB"
        );
        assert_eq!(
            converter.format_with_options(
                1000_u128.pow(12) * 2,
                &Options {
                    precision: 2,
                    separator: " ",
                    scientific_notation: true,
                }
            ),
            "2.00e12 YB"
        );
    }

    #[test]
    fn from_to() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: true,
                scale: Some(Scale::G),
            },
            to_unit: Unit {
                is_binary: true,
                scale: Some(Scale::M),
            },
        };

        assert_eq!(converter.format(64, 2), "65536.00 MiB");
        assert_eq!(converter.format(2, 2), "2048.00 MiB");
        assert_eq!(converter.format(128, 2), "131072.00 MiB");
        assert_eq!(converter.format(1024, 2), "1048576.00 MiB");
    }

    #[test]
    fn decimal_from_binary() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: Some(Scale::G),
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };

        assert_eq!(converter.format(22222, 2), "20.21 TiB");
        assert_eq!(converter.format(34_359_738_367, 2), "29.80 EiB");
    }

    #[test]
    fn from_binary() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: true,
                scale: Some(Scale::G),
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };

        assert_eq!(converter.format(1024, 2), "1.00 TiB");
        assert_eq!(converter.format(10240, 2), "10.00 TiB");
        assert_eq!(converter.format(512, 2), "512.00 GiB");
        assert_eq!(converter.format(10_000_000, 2), "9.54 PiB");
    }

    #[test]
    fn to_binary() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: true,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: Some(Scale::G),
            },
        };

        assert_eq!(converter.format(10_000_000, 2), "0.01 GiB");
        assert_eq!(converter.format(1024 * 512 * 1024 * 512, 2), "256.00 GiB");
        assert_eq!(
            converter.format(1024 * 111 * 1024 * 111 * 1024, 2),
            "12321.00 GiB"
        );
        assert_eq!(
            converter.format(1024 * 555 * 1024 * 555 * 1024, 2),
            "308025.00 GiB"
        );
    }

    #[test]
    fn with_options() {
        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };

        assert_eq!(
            converter.format_with_options(
                1024 * 1024 * 1024,
                &Options {
                    precision: 0,
                    separator: "",
                    ..Options::default()
                }
            ),
            "1GiB"
        );
        assert_eq!(
            converter.format_with_options(
                1_048_576,
                &Options {
                    precision: 2,
                    separator: "___",
                    ..Options::default()
                }
            ),
            "1.00___MiB"
        );
        assert_eq!(
            converter.format_with_options(1_048_576, &Options::default()),
            "1.0 MiB"
        );
    }
}
