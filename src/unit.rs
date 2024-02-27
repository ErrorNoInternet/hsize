use clap::ValueEnum;
use num_derive::FromPrimitive;
use std::fmt;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Unit {
    pub scale: Option<Scale>,
    pub is_binary: bool,
}

impl fmt::Display for Unit {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let scale = self.scale.unwrap_or_default();
        let mut output = format!("{scale:?}");
        if scale != Scale::B {
            if self.is_binary {
                output.push('i');
            }
            if !formatter.alternate() {
                output.push('B');
            }
        }
        write!(formatter, "{output}")
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, FromPrimitive, Ord, PartialEq, PartialOrd, ValueEnum)]
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
    R,
    Q,
}

impl Scale {
    pub const fn max_value() -> Self {
        Scale::Q
    }
}
