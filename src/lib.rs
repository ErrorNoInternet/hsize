use clap::ValueEnum;
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ValueEnum)]
pub enum SizeUnit {
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

pub struct Converter {
    pub binary: bool,
    pub precision: usize,
    pub from: Option<SizeUnit>,
    pub to: Option<SizeUnit>,
}

impl Converter {
    pub fn convert(&self, size: u128) -> String {
        let divisor = if self.binary { 1024 } else { 1000 } as f64;
        let mut current_size = size as f64;
        let mut current_unit = self.from.unwrap_or(SizeUnit::B);

        if let Some(to) = self.to {
            current_size *= divisor.powi(current_unit as i32);
            current_size /= divisor.powi(to as i32);
            current_unit = to;
        } else {
            while current_size >= divisor {
                if let Some(new_unit) = num_traits::FromPrimitive::from_u32(current_unit as u32 + 1)
                {
                    current_unit = new_unit;
                } else {
                    break;
                }
                current_size /= divisor;
            }
        }

        format!(
            "{:.*} {}",
            self.precision,
            current_size,
            self.display_unit(current_unit)
        )
    }

    pub fn display_unit(&self, unit: SizeUnit) -> String {
        let mut output = format!("{unit:?}");
        if unit != SizeUnit::B {
            output.push('B');
            if self.binary {
                output.insert(1, 'i');
            }
        }
        output
    }
}
