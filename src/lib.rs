pub mod format;
mod unit;

pub use unit::{Scale, Unit};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Converter {
    pub from_unit: Unit,
    pub to_unit: Unit,
}

impl Converter {
    pub fn convert(&self, size: u128) -> (f64, Scale) {
        #![allow(clippy::cast_possible_wrap)]

        #[allow(clippy::cast_precision_loss)]
        let mut new_size = size as f64; // TODO: switch to f128 (https://github.com/rust-lang/rust/pull/114607)

        let mut scale = self.from_unit.scale.unwrap_or_default();
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

        new_size *= from_divisor.powi(scale as i32);
        if let Some(to_scale) = self.to_unit.scale {
            new_size /= divisor.powi(to_scale as i32);
            scale = to_scale;
        } else {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_sign_loss)]
            let required_power = (new_size.log(divisor) as u32).clamp(0, Scale::max_value() as u32);
            new_size /= divisor.powi(required_power as i32);
            scale =
                num_traits::FromPrimitive::from_u32(required_power).unwrap_or(Scale::max_value());
        }

        (new_size, scale)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Converter, Scale, Unit};

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

        assert_eq!(converter.convert(1_048_576), (1.0, Scale::M));
        assert_eq!(converter.convert(1_048_576 * 100), (100.0, Scale::M));
        assert_eq!(converter.convert(1024 * 1024 * 1024 * 5), (5.0, Scale::G));
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

        assert_eq!(
            converter.convert(10_101_010_101_010_101),
            (10.101_010_101_010_1, Scale::P)
        );
        assert_eq!(converter.convert(123_456_789), (123.456_789, Scale::M));
        assert_eq!(
            converter.convert(1_111_111_111_111_111),
            (1.111_111_111_111_111, Scale::P)
        );
        assert_eq!(converter.convert(999_999), (999.999, Scale::K));
        assert_eq!(
            converter.convert(5_555_555_555_555_555_555_555_555_555_555),
            (5_555_555.555_555_556, Scale::Y)
        );
    }
}
