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

#[cfg(test)]
mod tests {
    use crate::{display_size, display_unit, SizeUnit};

    #[test]
    fn test_unit_conversions() {
        assert_eq!(
            num_traits::FromPrimitive::from_u32(SizeUnit::B as u32 + 1),
            Some(SizeUnit::K)
        );
        assert_eq!(
            num_traits::FromPrimitive::from_u32(SizeUnit::G as u32 - 1),
            Some(SizeUnit::M)
        );
        assert_eq!(
            num_traits::FromPrimitive::from_u32(SizeUnit::G as u32 + 1),
            Some(SizeUnit::T)
        );
        assert_eq!(
            num_traits::FromPrimitive::from_u32(SizeUnit::Z as u32 + 1),
            Some(SizeUnit::Y)
        );
        assert_eq!(
            num_traits::FromPrimitive::from_u32(SizeUnit::E as u32 + 1),
            Some(SizeUnit::Z)
        );
    }

    #[test]
    fn test_unit_outputs() {
        assert_eq!(display_unit(false, SizeUnit::B), "B");
        assert_eq!(display_unit(true, SizeUnit::B), "B");
        assert_eq!(display_unit(false, SizeUnit::G), "GB");
        assert_eq!(display_unit(true, SizeUnit::G), "GiB");
        assert_eq!(display_unit(false, SizeUnit::Y), "YB");
        assert_eq!(display_unit(true, SizeUnit::Y), "YiB");
    }

    #[test]
    fn test_size_outputs() {
        assert_eq!(display_size(1010, true, None, 2), "1010.00 B");
        assert_eq!(display_size(1010, false, None, 2), "1.01 KB");
        assert_eq!(display_size(123456789, false, None, 2), "123.46 MB");
        assert_eq!(display_size(1000000000000, false, None, 2), "1.00 TB");
        assert_eq!(display_size(1000000000000, true, None, 2), "931.32 GiB");
        assert_eq!(
            display_size(123456789, false, None, 10),
            "123.4567890000 MB"
        );
        assert_eq!(
            display_size(123456789, true, None, 10),
            "117.7375688553 MiB"
        );
        assert_eq!(
            display_size(123456789123456789123456789, true, None, 5),
            "102.12106 YiB"
        );
        assert_eq!(
            display_size(123456789123456789123456789, true, None, 5),
            "102.12106 YiB"
        );
        assert_eq!(
            display_size(123456789123456789123, true, Some(SizeUnit::M), 2),
            "117737568973023.22 MiB"
        );
        assert_eq!(
            display_size(1000000000000, false, Some(SizeUnit::M), 5),
            "1000000.00000 MB"
        );
        assert_eq!(
            display_size(1000000000000, true, Some(SizeUnit::M), 5),
            "953674.31641 MiB"
        );
        assert_eq!(
            display_size(1000000000000, true, Some(SizeUnit::M), 5),
            "953674.31641 MiB"
        );
        assert_eq!(
            display_size(1010101010101010101010101, false, Some(SizeUnit::Y), 5),
            "1.01010 YB"
        );
        assert_eq!(
            display_size(1010101010101010101010101, true, Some(SizeUnit::Y), 5),
            "0.83554 YiB"
        );
        assert_eq!(display_size(101010101, false, None, 0), "101 MB");
        assert_eq!(display_size(101010101, true, None, 0), "96 MiB");

        assert_eq!(display_size(69420, false, Some(SizeUnit::B), 0), "69420 B");
        assert_eq!(display_size(69420, true, Some(SizeUnit::B), 0), "69420 B");
        assert_eq!(
            display_size(10000000000000000000000, false, Some(SizeUnit::B), 0),
            "10000000000000000000000 B"
        );
        assert_eq!(
            display_size(10000000000000000000000, true, Some(SizeUnit::B), 0),
            "10000000000000000000000 B"
        );
    }
}
