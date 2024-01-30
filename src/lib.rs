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
    pub fn humanize(&self, size: u128) -> String {
        self.convert(size, SizeUnit::B)
    }

    pub fn convert(&self, size: u128, unit: SizeUnit) -> String {
        let divisor: u128 = if self.binary { 1024 } else { 1000 };
        let mut current_size = size as f64;
        let mut current_unit = unit;

        if let Some(unit) = self.to {
            current_unit = unit;
            current_size /= divisor.pow(current_unit as u32) as f64;
        } else {
            while current_size >= divisor as f64 {
                if let Some(new_unit) = num_traits::FromPrimitive::from_u32(current_unit as u32 + 1)
                {
                    current_unit = new_unit;
                } else {
                    break;
                }
                current_size /= divisor as f64;
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
