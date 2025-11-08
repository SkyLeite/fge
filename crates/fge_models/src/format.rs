use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

    pub max_health: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    /// The author's name
    pub name: String,

    /// The author's e-mail
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    /// A list of commands to be executed every frame when the character is in this state.
    pub commands: Vec<Command>,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateID(String);

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationID(String);

#[derive(Serialize, Deserialize)]
pub enum Action {
    /// Sets a new character state
    SetState(CharacterState),

    /// Sets the character's currently playing animation
    SetAnimation(AnimationID),

    /// Sets whether the character is currently being controlled
    SetControl(bool, bool),
}

#[derive(Serialize, Deserialize)]
pub enum CharacterState {
    Standing,
    Crouching,
    Airborne,
    Custom(StateID),
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    /// Action to run when this Command is executed
    pub action: Option<Action>,

    /// Condition under which this Command should be run. If the expression returns "false", the command is skipped
    pub condition: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Animation {
    Sprite(SpriteAnimation),
    Model(ModelAnimation),
}

/// An animation that consists of a sequence of 2D sprites
#[derive(Serialize, Deserialize)]
pub struct SpriteAnimation {
    pub frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub sprite: Sprite,
    pub duration: u32,
}

/// A single frame of an animation
#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub file: std::path::PathBuf,
}

/// An animation that uses a 3D model. Not currently implemented, and does nothing.
#[derive(Serialize, Deserialize)]
pub struct ModelAnimation {}
