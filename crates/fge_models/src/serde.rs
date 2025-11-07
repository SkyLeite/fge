use mlua::{Lua, LuaSerdeExt};

use crate::Character;

pub fn from_str(data: &str) -> Result<Character, Box<dyn std::error::Error>> {
    let lua = Lua::new();

    let val = lua.load(data).eval()?;
    let character: Character = lua.from_value(val)?;

    Ok(character)
}

pub fn from_file(path: &std::path::Path) -> Result<Character, Box<dyn std::error::Error>> {
    let lua_str = std::fs::read_to_string(path)?;
    from_str(&lua_str)
}

#[cfg(test)]
mod test {
    use fixtures::fixtures;

    #[fixtures(["basic.lua"])]
    #[test]
    pub fn basic(path: &std::path::Path) {
        let character = super::from_file(path).unwrap();

        assert_eq!(character.id, "sf4viper");
        assert_eq!(character.name, "C. Viper");
    }
}
