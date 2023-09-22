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
    let mut output = format!("{:?}", unit);
    if unit != SizeUnit::B {
        output.push('B');
        if binary {
            output.insert(1, 'i')
        }
    }
    output
}

pub fn display_size(size: u128, binary: bool, unit: Option<SizeUnit>, precision: usize) -> String {
    let divisor: u128 = if binary { 1024 } else { 1000 };
    let mut current_size = size as f64;
    let mut current_unit = SizeUnit::B;

    if unit.is_none() {
        while current_size >= divisor as f64 {
            if let Some(new_unit) = num_traits::FromPrimitive::from_u32(current_unit as u32 + 1) {
                current_unit = new_unit
            } else {
                break;
            }
            current_size = current_size / divisor as f64;
        }
    } else {
        current_unit = unit.unwrap();
        current_size = current_size / divisor.pow(current_unit as u32) as f64
    }
    format!(
        "{:.precision$} {}",
        current_size,
        display_unit(binary, current_unit)
    )
}
