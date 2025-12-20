use fge_input::Inputs;
use serde::{Deserialize, Serialize};

use crate::Step;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "StepSetSerde")]
pub struct StepSet {
    /// List of steps in this StepSet
    steps: Vec<Step>,

    #[doc(hidden)]
    resolved: Vec<Inputs>,

    /// How many frames the user has to input this StepSet
    pub buffer_window_size: u16,
}

#[allow(unused)]
fn default_buffer_window_size() -> u16 {
    30
}

impl StepSet {
    pub fn new(steps: Vec<Step>, buffer_window_size: u16) -> Self {
        Self {
            resolved: Self::resolve(&steps),
            steps,
            buffer_window_size,
        }
    }

    fn resolve(steps: &Vec<Step>) -> Vec<Inputs> {
        let mut out: Vec<Inputs> = vec![];

        for step in steps {
            match step {
                Step::Press { inputs } => out.push(inputs.clone()),
                Step::Hold { inputs, min_frames } => {
                    for _ in 0..*min_frames {
                        out.push(inputs.clone());
                    }
                }
            }
        }

        out
    }
}

impl<'a> IntoIterator for &'a StepSet {
    type Item = &'a Inputs;

    type IntoIter = std::slice::Iter<'a, Inputs>;

    fn into_iter(self) -> Self::IntoIter {
        self.resolved.iter()
    }
}

#[derive(Debug, Deserialize)]
struct StepSetSerde {
    steps: Vec<Step>,

    #[serde(default = "default_buffer_window_size")]
    buffer_window_size: u16,
}

impl From<StepSetSerde> for StepSet {
    fn from(value: StepSetSerde) -> Self {
        Self {
            resolved: StepSet::resolve(&value.steps),
            steps: value.steps,
            buffer_window_size: value.buffer_window_size,
        }
    }
}
