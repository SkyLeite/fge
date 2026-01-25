use bevy::prelude::*;
use enumflags2::bitflags;

#[bitflags]
#[repr(u16)]
#[derive(Reflect, Debug, Copy, Clone)]
pub enum Input {
    /// Up (8)
    U,

    /// Forward (6)
    F,

    /// Down (2)
    D,

    /// Back (4)
    B,

    /// Attack 1
    A1,

    /// Attack 2
    A2,

    /// Attack 3
    A3,

    /// Attack 4
    A4,
}

impl TryFrom<&str> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "8" => Ok(Self::U),
            "6" => Ok(Self::F),
            "2" => Ok(Self::D),
            "4" => Ok(Self::B),
            "A" => Ok(Self::A1),
            "B" => Ok(Self::A2),
            "C" => Ok(Self::A3),
            "D" => Ok(Self::A4),
            _ => Err(todo!()),
        }
    }
}

impl TryFrom<&String> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
