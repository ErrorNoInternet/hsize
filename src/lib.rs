use clap::ValueEnum;
use num_derive::FromPrimitive;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Unit {
    pub scale: Option<Scale>,
    pub binary: bool,
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        let scale = self.scale.unwrap_or_default();
        let mut output = format!("{scale:?}");
        if scale != Scale::B {
            output.push('B');
            if self.binary {
                output.insert(1, 'i');
            }
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
                binary: characters[1] == 'i',
                scale: Some(characters[0].into()),
            }
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive, ValueEnum)]
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

pub struct Converter {
    pub precision: usize,
    pub from_unit: Unit,
    pub to_unit: Unit,
}

impl Converter {
    pub fn convert(&self, size: u128) -> String {
        // TODO: switch to f128 (https://github.com/rust-lang/rust/pull/114607)
        #[allow(clippy::cast_precision_loss)]
        let mut current_size = size as f64;

        let mut current_scale = self.from_unit.scale.unwrap_or_default();
        let from_divisor: f64 = if self.from_unit.binary {
            1024.0
        } else {
            1000.0
        };
        let to_divisor: f64 = if self.to_unit.binary { 1024.0 } else { 1000.0 };

        if let Some(to_scale) = self.to_unit.scale {
            current_size *= from_divisor.powi(current_scale as i32);
            current_size /= to_divisor.powi(to_scale as i32);
            current_scale = to_scale;
        } else {
            while current_size >= to_divisor {
                if let Some(new_scale) =
                    num_traits::FromPrimitive::from_u32(current_scale as u32 + 1)
                {
                    current_scale = new_scale;
                } else {
                    break;
                }
                current_size /= to_divisor;
            }
        }

        format!(
            "{:.*} {}",
            self.precision,
            current_size,
            Unit {
                binary: self.to_unit.binary,
                scale: Some(current_scale),
            }
            .to_string()
        )
    }
}
