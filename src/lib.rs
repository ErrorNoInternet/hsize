use clap::ValueEnum;
use num_derive::FromPrimitive;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Unit {
    pub scale: Option<Scale>,
    pub is_binary: bool,
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        let scale = self.scale.unwrap_or_default();
        let mut output = format!("{scale:?}");
        if scale != Scale::B {
            if self.is_binary {
                output.push('i');
            }
            output.push('B');
        }
        output
    }
}

impl FromStr for Unit {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let characters = string.chars().collect::<Vec<_>>();
        Ok(if characters.len() < 2 {
            Unit::default()
        } else {
            Self {
                is_binary: characters[1] == 'i',
                scale: Some(characters[0].into()),
            }
        })
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, FromPrimitive, PartialEq, ValueEnum)]
pub enum Scale {
    #[default]
    B,
    K,
    M,
    G,
    T,
    P,
    E,
    Z,
    Y,
}

impl From<char> for Scale {
    fn from(character: char) -> Self {
        match character {
            'k' | 'K' => Scale::K,
            'm' | 'M' => Scale::M,
            'g' | 'G' => Scale::G,
            't' | 'T' => Scale::T,
            'p' | 'P' => Scale::P,
            'e' | 'E' => Scale::E,
            'z' | 'Z' => Scale::Z,
            'y' | 'Y' => Scale::Y,
            _ => Scale::B,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Converter {
    pub precision: usize,
    pub from_unit: Unit,
    pub to_unit: Unit,
}

impl Converter {
    pub fn convert(&self, size: u128) -> (f64, Scale) {
        // TODO: switch to f128 (https://github.com/rust-lang/rust/pull/114607)
        #[allow(clippy::cast_precision_loss)]
        let mut new_size = size as f64;

        let mut new_scale = self.from_unit.scale.unwrap_or_default();
        let from_divisor: f64 = if self.from_unit.is_binary {
            1024.0
        } else {
            1000.0
        };
        let divisor: f64 = if self.to_unit.is_binary {
            1024.0
        } else {
            1000.0
        };

        if let Some(to_scale) = self.to_unit.scale {
            new_size *= from_divisor.powi(new_scale as i32);
            new_size /= divisor.powi(to_scale as i32);
            new_scale = to_scale;
        } else {
            while new_size >= divisor {
                if let Some(next_scale) = num_traits::FromPrimitive::from_u32(new_scale as u32 + 1)
                {
                    new_scale = next_scale;
                } else {
                    break;
                }
                new_size /= divisor;
            }
        }

        (new_size, new_scale)
    }

    pub fn format(&self, size: u128) -> String {
        let (new_size, new_scale) = self.convert(size);
        format!(
            "{:.*} {}",
            self.precision,
            new_size,
            Unit {
                is_binary: self.to_unit.is_binary,
                scale: Some(new_scale),
            }
            .to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Converter, Scale, Unit};

    #[test]
    fn basic() {
        let converter = Converter {
            precision: 0,
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };

        assert_eq!(converter.format(0), "0 B");
        assert_eq!(converter.format(123), "123 B");
        assert_eq!(converter.format(5555), "5 KiB");
        assert_eq!(converter.format(1_048_576), "1 MiB");
        assert_eq!(converter.format(1024 * 1024 * 1024), "1 GiB");
    }

    #[test]
    fn big() {
        let converter = Converter {
            precision: 5,
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };

        assert_eq!(converter.format(10_101_010_101_010_101), "10.10101 PB");
        assert_eq!(converter.format(123_456_789), "123.45679 MB");
        assert_eq!(converter.format(1_111_111_111_111_111_111), "1.11111 EB");
        assert_eq!(converter.format(999_999), "999.99900 KB");
        assert_eq!(
            converter.format(5_555_555_555_555_555_555_555_555_555_555),
            "5555555.55556 YB"
        );
    }

    #[test]
    fn from_to() {
        let converter = Converter {
            precision: 2,
            from_unit: Unit {
                is_binary: true,
                scale: Some(Scale::G),
            },
            to_unit: Unit {
                is_binary: true,
                scale: Some(Scale::M),
            },
        };

        assert_eq!(converter.format(64), "65536.00 MiB");
        assert_eq!(converter.format(2), "2048.00 MiB");
        assert_eq!(converter.format(128), "131072.00 MiB");
        assert_eq!(converter.format(1024), "1048576.00 MiB");
    }

    #[test]
    fn from() {
        let converter = Converter {
            precision: 2,
            from_unit: Unit {
                is_binary: true,
                scale: Some(Scale::G),
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };

        assert_eq!(converter.format(1024), "1.00 TiB");
        assert_eq!(converter.format(10240), "10.00 TiB");
        assert_eq!(converter.format(512), "512.00 GiB");
        assert_eq!(converter.format(10_000_000), "9.54 PiB");
    }

    #[test]
    fn to() {
        let converter = Converter {
            precision: 2,
            from_unit: Unit {
                is_binary: true,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: Some(Scale::G),
            },
        };

        assert_eq!(converter.format(10_000_000), "0.01 GiB");
        assert_eq!(converter.format(1024 * 512 * 1024 * 512), "256.00 GiB");
        assert_eq!(
            converter.format(1024 * 111 * 1024 * 111 * 1024),
            "12321.00 GiB"
        );
        assert_eq!(
            converter.format(1024 * 555 * 1024 * 555 * 1024),
            "308025.00 GiB"
        );
    }
}
