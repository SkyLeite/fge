use mlua::{Lua, LuaSerdeExt, Table};

use crate::{Game};

pub fn from_str(lua: &Lua, data: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let val = lua.load(data).eval()?;
    let game: Game = lua.from_value(val)?;

    Ok(game)
}

pub fn from_file(path: &std::path::Path) -> Result<Game, Box<dyn std::error::Error>> {
    let base_game_folder = if path.is_dir() {
        path.canonicalize().unwrap()
    } else {
        path.parent().and_then(|f| f.canonicalize().ok()).unwrap()
    };

    let base_game_folder_str = base_game_folder.to_string_lossy();
    let lua_str = std::fs::read_to_string(path)?;

    let searchpath = [
        "./?.lua",
        "./?",
        &format!("{}/?.lua", base_game_folder_str),
        &format!("{}/?/init.lua", base_game_folder_str),
    ].join(";");

    let lua = Lua::new();
    let globals = lua.globals();
    let package = globals.get::<Table>("package").unwrap();
    package.set("path", searchpath).unwrap();

    globals.set("package", package).unwrap();

    lua.set_globals(globals).unwrap();

    from_str(&lua, &lua_str)
}

#[cfg(test)]
mod test {
    use fixtures::fixtures;

    use crate::CharacterID;

    #[fixtures(["basic.lua"])]
    #[test]
    pub fn basic(path: &std::path::Path) {
        let game = super::from_file(path).unwrap();

        assert!(game.characters.contains_key(&CharacterID("akiha".into())));
    }
}
