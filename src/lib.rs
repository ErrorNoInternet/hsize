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

pub fn display_unit(binary: bool, unit: SizeUnit) -> String {
    let mut output = format!("{unit:?}");
    if unit != SizeUnit::B {
        output.push('B');
        if binary {
            output.insert(1, 'i');
        }
    }
    output
}

pub fn display_size(size: u128, binary: bool, unit: Option<SizeUnit>, precision: usize) -> String {
    let divisor: u128 = if binary { 1024 } else { 1000 };
    let mut current_size = size as f64;
    let mut current_unit = SizeUnit::B;

    if let Some(unit) = unit {
        current_unit = unit;
        current_size /= divisor.pow(current_unit as u32) as f64;
    } else {
        while current_size >= divisor as f64 {
            if let Some(new_unit) = num_traits::FromPrimitive::from_u32(current_unit as u32 + 1) {
                current_unit = new_unit;
            } else {
                break;
            }
            current_size /= divisor as f64;
        }
    }
    format!(
        "{:.precision$} {}",
        current_size,
        display_unit(binary, current_unit)
    )
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
