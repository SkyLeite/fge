use enumflags2::{BitFlag, BitFlags, bitflags};
use ringbuffer::ConstGenericRingBuffer;

use crate::prelude::*;

#[derive(Component, Debug, Default)]
pub struct InputHistory {
    pub history: ConstGenericRingBuffer<BitFlags<Input>, 16>,
}

impl InputHistory {
    pub fn push(&mut self, input: BitFlags<Input>) {
        self.history.enqueue(input);
    }
}

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
}

impl Input {
    pub fn to_bitflags(input: &ButtonInput<KeyCode>) -> BitFlags<Input> {
        let mut initial = Input::empty();

        if input.just_pressed(KeyCode::ArrowUp) {
            initial = initial | Input::U
        }

        if input.just_pressed(KeyCode::ArrowRight) {
            initial = initial | Input::F;
        }

        if input.just_pressed(KeyCode::ArrowLeft) {
            initial = initial | Input::B;
        }

        if input.just_pressed(KeyCode::ArrowDown) {
            initial = initial | Input::D;
        }

        initial
    }
}
