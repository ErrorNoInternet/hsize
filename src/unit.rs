use clap::ValueEnum;
use num_derive::FromPrimitive;

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
}

impl Scale {
    pub const fn max_value() -> Self {
        Scale::Y
    }
}
