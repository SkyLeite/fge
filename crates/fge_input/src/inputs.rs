use std::{
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut},
    str::FromStr,
};

use bevy::prelude::*;
use enumflags2::BitFlags;
use serde::Serialize;
use serde_with::DeserializeFromStr;

use crate::input::Input;

#[derive(Debug, Clone, Serialize, DeserializeFromStr, Default, Eq, PartialEq)]
#[serde(try_from = "&str")]
pub struct Inputs(BitFlags<Input>);

impl Inputs {
    pub fn new(input: BitFlags<Input>) -> Self {
        Self(input)
    }

    pub fn single(input: Input) -> Self {
        Self(input.into())
    }

    pub fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn all() -> Self {
        Self(BitFlags::ALL)
    }
}

impl Deref for Inputs {
    type Target = BitFlags<Input>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Inputs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Input> for Inputs {
    fn from(input: Input) -> Self {
        Self(input.into())
    }
}

impl From<BitFlags<Input>> for Inputs {
    fn from(value: BitFlags<Input>) -> Self {
        Self(value)
    }
}

impl From<&ButtonInput<KeyCode>> for Inputs {
    fn from(input: &ButtonInput<KeyCode>) -> Self {
        let mut initial = Self::empty();

        if input.pressed(KeyCode::ArrowUp) {
            initial |= Input::U;
        }

        if input.pressed(KeyCode::ArrowRight) {
            initial |= Input::F;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            initial |= Input::B;
        }

        if input.pressed(KeyCode::ArrowDown) {
            initial |= Input::D;
        }

        if input.pressed(KeyCode::KeyA) {
            initial |= Input::A1;
        }

        if input.pressed(KeyCode::KeyS) {
            initial |= Input::A2;
        }

        if input.pressed(KeyCode::KeyD) {
            initial |= Input::A3;
        }

        if input.pressed(KeyCode::KeyF) {
            initial |= Input::A4;
        }

        // Clean up SOCD inputs
        if initial.contains(Input::U) && initial.contains(Input::D) {
            initial ^= Input::U;
            initial ^= Input::D;
        }

        if initial.contains(Input::F) && initial.contains(Input::B) {
            initial ^= Input::F;
            initial ^= Input::B;
        }

        initial
    }
}

impl FromStr for Inputs {
    type Err = String;

    fn from_str(token: &str) -> std::result::Result<Self, Self::Err> {
        let s = token.trim();
        if s.is_empty() {
            return Err("Input string is empty".into());
        }

        let mut out = BitFlags::EMPTY;
        let parts = token.split("+").map(|p| p.trim()).filter(|p| !p.is_empty());

        for part in parts {
            let input = match part {
                "1" => Ok(Input::D | Input::B),
                "3" => Ok(Input::D | Input::F),
                "9" => Ok(Input::U | Input::F),
                "7" => Ok(Input::U | Input::B),
                other => Input::try_from(other).map(BitFlags::from_flag),
            }
            .map_err(|_e| "Failed".to_owned())?;

            out |= input;
        }

        Ok(Self(out))
    }
}

// Bitwise operations
impl BitOrAssign<Input> for Inputs {
    fn bitor_assign(&mut self, rhs: Input) {
        self.0 |= rhs
    }
}

impl BitOr<Input> for Inputs {
    type Output = Self;

    fn bitor(self, rhs: Input) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitAnd<Input> for Inputs {
    type Output = Self;

    fn bitand(self, rhs: Input) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAndAssign<Input> for Inputs {
    fn bitand_assign(&mut self, rhs: Input) {
        self.0 &= rhs
    }
}

impl BitXor<Input> for Inputs {
    type Output = Self;

    fn bitxor(self, rhs: Input) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXorAssign<Input> for Inputs {
    fn bitxor_assign(&mut self, rhs: Input) {
        self.0 ^= rhs
    }
}
