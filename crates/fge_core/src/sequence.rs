use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::Animation;
use fge_models::{Frame, Square};

pub struct Sequence {
    pub animation: Handle<Animation>,
    pub default_collision_box: Option<Square>,
    pub frames: Vec<Frame>,
}

impl Sequence {
    fn get_frame_p(frames: &Vec<Frame>, index: u32) -> Option<&Frame> {
        let mut count = 1;

        for frame in frames {
            count += frame.duration;

            if index <= count {
                return Some(frame);
            }
        }

        None
    }

    /// Given an animation frame number, returns the Frame of this sequence that corresponds to it.
    pub fn get_frame(&self, index: u32) -> Option<&Frame> {
        Sequence::get_frame_p(&self.frames, index)
    }
}

#[cfg(test)]
mod test {
    use fge_models::Frame;
    use rstest::rstest;

    use crate::sequence::Sequence;

    #[rstest]
    #[case(0, [2, 2, 2, 2], Some("id-0"))]
    #[case(1, [2, 2, 2, 2], Some("id-0"))]
    #[case(3, [2, 2, 2, 2], Some("id-0"))]
    #[case(4, [2, 2, 2, 2], Some("id-1"))]
    #[case(5, [2, 2, 2, 2], Some("id-1"))]
    #[case(6, [2, 2, 2, 2], Some("id-2"))]
    #[case(7, [2, 2, 2, 2], Some("id-2"))]
    #[case(8, [2, 2, 2, 2], Some("id-3"))]
    fn get_frame_returns_correctly_for_index(
        #[case] index: u32,
        #[case] frame_durations: [u32; 4],
        #[case] expected: Option<&str>,
    ) {
        let frames = frame_durations
            .iter()
            .enumerate()
            .map(|(index, duration)| Frame {
                sheet: format!("id-{index}").into(),
                cell: (1, 1),
                duration: *duration,
            })
            .collect();

        let out = Sequence::get_frame_p(&frames, index).map(|f| f.sheet.0.as_str());

        assert_eq!(out, expected, "Got id {:?}, expected {:?}", out, expected);
    }
}
