use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Deref, path::PathBuf};

use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize)]
pub struct Character {
    /// A unique identifier for this character.
    /// Example: `mvc2cable`
    pub id: String,

    /// The character's canonical name. This is what's shown to the player in-game.
    /// Example: `Cable`
    pub name: String,

    /// Author information
    pub author: Option<Author>,

    /// The character's states
    pub states: HashMap<StateID, State>,

    /// The character's animations
    pub animations: HashMap<AnimationID, Animation>,

    /// A list of spritesheets to be used for animations and effects
    pub spritesheets: HashMap<SpritesheetID, Spritesheet>,

    pub max_health: u32,
}

#[derive(Reflect, Debug, Serialize, Deserialize)]
pub struct Spritesheet {
    /// Path to file containing the spritesheet
    pub file: PathBuf,

    /// Amount of columns in the spritesheet
    pub columns: u16,

    /// Amount of rows in the spritesheet
    pub rows: u16,

    /// Spritesheet width in pixels
    pub width: u16,

    /// Spritesheet height in pixels
    pub height: u16,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Author {
    /// The author's name
    pub name: String,

    /// The author's e-mail
    pub email: String,
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub struct State {
    /// A list of commands to be executed every frame when the character is in this state.
    pub commands: Vec<Command>,

    /// States in which this state can be canceled into
    pub cancels: HashMap<StateID, Cancel>,

    /// Player input used to transition to this state
    pub input: Option<String>,
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub struct Condition(String);

impl Deref for Condition {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub struct Cancel {
    /// Frame range in which this cancel window is valid. If omitted, the cancel window is always valid
    pub frames: Option<NumberOrRange>,

    /// Extra conditions to make this cancel window valid
    pub condition: Option<Condition>,
}

#[derive(Reflect, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateID(String);

impl Display for StateID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for StateID {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for StateID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Reflect, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AnimationID(String);

impl Display for AnimationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for AnimationID {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for AnimationID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Reflect, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct SpritesheetID(pub String);

impl From<&str> for SpritesheetID {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for SpritesheetID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Sets a new character state. If there's an animation with the same name as the state, it is also set
    SetState(CharacterState),

    /// Sets the character's currently playing animation
    SetAnimation(AnimationID),

    /// Sets whether the character is currently being controlled
    SetControl(bool, bool),

    /// Sets the hitboxes for this frame
    SetHitboxes(Vec<Square>),
}

#[derive(Reflect, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum CharacterState {
    #[default]
    Standing,
    Crouching,
    Airborne,
    Custom(StateID),
}

impl ToString for CharacterState {
    fn to_string(&self) -> String {
        match self {
            CharacterState::Standing => "standing".to_owned(),
            CharacterState::Crouching => "crouching".to_owned(),
            CharacterState::Airborne => "airborne".to_owned(),
            CharacterState::Custom(state_id) => state_id.to_string(),
        }
    }
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Action to run when this Command is executed
    pub action: Action,

    /// Condition under which this Command should be run. If the expression returns "false", the command is skipped
    pub condition: Option<Condition>,

    /// Range of frames to run this command during
    pub frames: Option<NumberOrRange>,
}

#[derive(Reflect, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberOrRange {
    Number(u32),
    Range(Range),
}

/// A (half-open) range bounded inclusively below and exclusively above (from..to).
/// The range from..to contains all values with from <= x < to. It is empty if from >= to.
#[derive(Reflect, Copy, Clone, Serialize, Deserialize)]
pub struct Range {
    pub from: u32,
    pub to: u32,
}

#[derive(Reflect, Serialize, Deserialize)]
pub enum Animation {
    Sprite(SpriteAnimation),
    Model(ModelAnimation),
}

/// An animation that consists of a sequence of 2D sprites
#[derive(Reflect, Clone, Serialize, Deserialize)]
pub struct SpriteAnimation {
    /// List of frames to be shown
    pub frames: Vec<Frame>,

    /// Default collision box to use for all Frames. This can be overriden per-frame.
    pub default_collision_box: Option<Square>,
}

#[derive(Reflect, Clone, Serialize, Deserialize)]
/// A generic Box, used to represent collision, hit and hurtboxes.
pub struct Square {
    /// X position of the center point
    pub x: i32,

    /// Y position of the center point
    pub y: i32,

    /// Width of the box, in pixels
    pub w: u32,

    /// Height of the box, in pixels
    pub h: u32,
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct Frame {
    /// ID of the spritesheet to use for this frame. If omitted, defaults to the animation's name
    pub sheet: Option<SpritesheetID>,

    /// Index of the sprite in the spritesheet
    pub cell: (u16, u16),

    /// Amount of in-game frames to show this animation frame for
    pub duration: u32,
}

/// A single frame of an animation
#[derive(Reflect, Serialize, Deserialize)]
pub struct Sprite {
    pub file: std::path::PathBuf,
}

/// An animation that uses a 3D model. Not currently implemented, and does nothing.
#[derive(Reflect, Serialize, Deserialize)]
pub struct ModelAnimation {}
