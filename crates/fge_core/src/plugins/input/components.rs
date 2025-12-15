use enumflags2::{BitFlag, BitFlags, bitflags};
use ringbuffer::ConstGenericRingBuffer;

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct InputHistory {
    pub history: ConstGenericRingBuffer<BitFlags<Input>, 16>,
}

impl Default for InputHistory {
    fn default() -> Self {
        let mut out = Self {
            history: ConstGenericRingBuffer::new(),
        };

        out.history.fill_default();

        out
    }
}

#[allow(dead_code)]
impl InputHistory {
    /// Adds a new input to the input history
    pub fn push(&mut self, input: BitFlags<Input>) {
        self.history.enqueue(input);
    }

    /// Returns true if the given input was released on this frame
    ///
    /// More specifically, returns true if the previous frame contains the input, but the current doesn't
    pub fn just_released(&self, key: Input) -> bool {
        !self.history[15].contains(key) && self.history[14].contains(key)
    }

    /// Returns true if the given input was first pressed on this frame
    ///
    /// More specifically, returns true if the previous frame does not contain the input, but the current does
    pub fn just_pressed(&self, key: Input) -> bool {
        self.history[15].contains(key) && !self.history[14].contains(key)
    }

    /// Returns true if the given input was pressed on this frame
    pub fn pressed(&self, key: Input) -> bool {
        self.history[15].contains(key)
    }

    /// Returns true if the given input is being held.
    ///
    /// More specifically, returns true if both the current and previous frames contain the input
    pub fn is_held(&self, key: Input) -> bool {
        self.history[15].contains(key) && self.history[14].contains(key)
    }

    /// Returns the most recent input frame
    pub fn last(&self) -> &BitFlags<Input> {
        // Unwrap because we guarantee the buffer is always full by initializing with fill_default()
        self.history.back().unwrap()
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

    /// Attack 1
    A1,

    /// Attack 2
    A2,

    /// Attack 3
    A3,

    /// Attack 4
    A4,
}

impl Input {
    pub fn to_bitflags(input: &ButtonInput<KeyCode>) -> BitFlags<Input> {
        let mut initial = Input::empty();

        if input.pressed(KeyCode::ArrowUp) {
            initial = initial | Input::U
        }

        if input.pressed(KeyCode::ArrowRight) {
            initial = initial | Input::F;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            initial = initial | Input::B;
        }

        if input.pressed(KeyCode::ArrowDown) {
            initial = initial | Input::D;
        }

        if input.pressed(KeyCode::KeyA) {
            initial = initial | Input::A1;
        }

        // Clean up SOCD inputs
        if initial.contains(Input::U) && initial.contains(Input::D) {
            initial = initial ^ Input::U;
            initial = initial ^ Input::D;
        }

        if initial.contains(Input::F) && initial.contains(Input::B) {
            initial = initial ^ Input::F;
            initial = initial ^ Input::B;
        }

        initial
    }
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
            _ => Err(Box::new(TextError::NoSuchFont)),
        }
    }
}

impl TryFrom<&String> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod test {
    use enumflags2::BitFlags;

    use crate::plugins::input::{Input, components::InputHistory};

    #[test]
    fn just_pressed() {
        let key = Input::D;
        let mut history = InputHistory::default();

        assert_eq!(history.just_pressed(key), false);

        history.push(key.into());

        assert!(
            history.just_pressed(key),
            "history.just_pressed() must return true on the frame the key was added"
        );
    }

    #[test]
    fn just_released() {
        let key = Input::D;
        let mut history = InputHistory::default();

        assert_eq!(history.just_released(key), false);

        history.push(key.into());
        history.push(BitFlags::empty());

        assert!(
            history.just_released(key),
            "history.just_released() must return true on the frame the key was added"
        );
    }
}
