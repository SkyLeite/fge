use fge_input::{Input, Inputs};
use itertools::Itertools;
use ringbuffer::ConstGenericRingBuffer;

use crate::prelude::*;

#[derive(Component)]
pub struct InputHistory {
    pub history: ConstGenericRingBuffer<Inputs, 999>,
}

impl std::fmt::Debug for InputHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();

        for entry in &self.history {
            list.entry(entry);
        }

        list.finish()
    }
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
    pub fn push(&mut self, inputs: Inputs) {
        self.history.enqueue(inputs);
    }

    /// Returns true if the given input was released on this frame
    ///
    /// More specifically, returns true if the previous frame contains the input, but the current doesn't
    pub fn just_released(&self, key: Input) -> bool {
        let len = self.history.len();
        !self.history[len - 1].contains(key) && self.history[len - 2].contains(key)
    }

    /// Returns true if the given input was first pressed on this frame
    ///
    /// More specifically, returns true if the previous frame does not contain the input, but the current does
    pub fn just_pressed(&self, key: Input) -> bool {
        let len = self.history.len();
        self.history[len - 1].contains(key) && !self.history[len - 2].contains(key)
    }

    /// Returns true if the given input was pressed on this frame
    pub fn pressed(&self, key: Input) -> bool {
        let len = self.history.len();
        self.history[len - 1].contains(key)
    }

    /// Returns true if the given input is being held.
    ///
    /// More specifically, returns true if both the current and previous frames contain the input
    pub fn is_held(&self, key: Input) -> bool {
        let len = self.history.len();
        self.history[len - 1].contains(key) && self.history[len - 2].contains(key)
    }

    /// Returns the most recent input frame
    pub fn last(&self) -> &Inputs {
        // Unwrap because we guarantee the buffer is always full by initializing with fill_default()
        self.history.back().unwrap()
    }

    /// Returns whether the sequence is contained in the head of the history
    pub fn matches<'a, T>(&self, sequence: T, window_size: usize) -> bool
    where
        T: IntoIterator<Item = &'a Inputs>,
        <T as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        let window = self
            .history
            .iter()
            .rev()
            .take(window_size) // Take only the specified window_size
            .dedup()
            .filter(|i| !i.is_empty()); // Ignore neutral inputs

        let mut sequence_iter = sequence.into_iter().rev();

        // All entries in input history exist in the sequence
        let all_matches = !window
            .zip(sequence_iter.by_ref())
            .any(|(i, j)| !i.contains(**j));

        // Are there any elements in sequence_iter that didn't match the input history?
        sequence_iter.next().is_none() && all_matches
    }

    /// Returns whether the sequence is contained anywhere in the history
    pub fn contains() -> bool {
        todo!("Implement InputHistory::contains");
    }

    /// Returns a condensed version of the input history, where each entry is a tuple of inputs and the amount of frames it was held for
    pub fn condensed(&self) -> Vec<(&Inputs, i32)> {
        self.history.iter().rev().fold(Vec::new(), |mut acc, x| {
            if let Some(last) = &mut acc.last_mut()
                && last.0 == x
            {
                last.1 += 1;
            } else {
                acc.push((x, 1));
            }

            acc
        })
    }
}

#[cfg(test)]
mod test {
    use crate::plugins::input::components::InputHistory;
    use fge_input::{Input, Inputs};
    use fge_models::StepSet;

    #[test]
    fn just_pressed() {
        let key = Input::U;
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
        history.push(Inputs::empty());

        assert!(
            history.just_released(key),
            "history.just_released() must return true on the frame the key was added"
        );
    }

    #[test]
    fn matches_happy_path() {
        let mut history = InputHistory::default();
        history.push(Input::D.into());
        history.push((Input::D | Input::F).into());
        history.push(Input::F.into());
        history.push(Input::A1.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let buffer_window = 16;
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert!(matches);
    }

    #[test]
    fn matches_neutral_inputs_between() {
        let mut history = InputHistory::default();
        history.push(Input::D.into());
        history.push(Inputs::empty());
        history.push((Input::D | Input::F).into());
        history.push(Inputs::empty());
        history.push(Input::F.into());
        history.push(Inputs::empty());
        history.push(Input::A1.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let buffer_window = 16;
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert!(matches);
    }

    #[test]
    fn matches_neutral_inputs_before() {
        let mut history = InputHistory::default();
        history.push(Input::F.into());
        history.push(Input::U.into());
        history.push(Input::D.into());
        history.push(Input::D.into());
        history.push((Input::D | Input::F).into());
        history.push(Input::F.into());
        history.push(Input::A1.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let buffer_window = 16;
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert!(matches);
    }

    #[test]
    fn matches_neutral_inputs_after() {
        let mut history = InputHistory::default();
        history.push(Input::D.into());
        history.push((Input::D | Input::F).into());
        history.push(Input::F.into());
        history.push(Input::A1.into());
        history.push(Input::D.into());
        history.push(Input::F.into());
        history.push(Input::U.into());
        history.push(Input::U.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let buffer_window = 16;
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert_eq!(matches, false);
    }

    #[test]
    fn matches_inputs_no_match() {
        let mut history = InputHistory::default();
        history.push(Input::D.into());
        history.push((Input::D | Input::F).into());
        history.push(Input::F.into());
        history.push(Input::A2.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let buffer_window = 16;
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert_eq!(matches, false);
    }

    #[test]
    fn matches_buffer_window_outside() {
        let buffer_window = 16;
        let mut history = InputHistory::default();

        history.push(Input::D.into());
        for _ in 0..buffer_window {
            history.push(Inputs::empty());
        }

        history.push((Input::D | Input::F).into());
        history.push(Input::F.into());
        history.push(Input::A1.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert_eq!(matches, false);
    }

    #[test]
    fn matches_allows_repeats() {
        let repeat_count = 5;
        let mut history = InputHistory::default();

        for _ in 0..repeat_count {
            history.push(Input::D.into());
        }

        for _ in 0..repeat_count {
            history.push((Input::D | Input::F).into());
        }

        for _ in 0..repeat_count {
            history.push(Input::F.into());
        }

        history.push(Input::A1.into());

        // 236A
        let steps = vec![
            fge_models::Step::Press {
                inputs: Input::D.into(),
            },
            fge_models::Step::Press {
                inputs: (Input::D | Input::F).into(),
            },
            fge_models::Step::Press {
                inputs: Input::F.into(),
            },
            fge_models::Step::Press {
                inputs: Input::A1.into(),
            },
        ];
        let buffer_window = 30;
        let step_set = StepSet::new(steps, buffer_window);
        let iter = step_set.into_iter();

        let matches = history.matches(iter, buffer_window as usize);

        assert!(matches);
    }
}
