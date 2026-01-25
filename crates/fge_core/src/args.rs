use std::path::PathBuf;

use crate::prelude::*;
use argh::FromArgs;

/// MetaDeclare fighting game engine
#[derive(Resource, FromArgs)]
pub struct Args {
    /// specify a path to the game's script
    #[argh(option, default = "default_path()")]
    pub game_path: PathBuf,
}

fn default_path() -> PathBuf {
    let search_paths = ["./data"];

    search_paths
        .map(|path| PathBuf::from(path))
        .iter()
        .find(|path| path.exists())
        .expect("Game Path not specified and default search paths not found.")
        .to_path_buf()
}

impl Args {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}
