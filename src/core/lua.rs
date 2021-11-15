use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;

const NEOSH_STDLIB: &str = include_str!("../lua/neosh.lua");

// Initialize Lua globals
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    // ===== Setup package path so we can require scripts
    lua.load(
        r#"
        -- Get the system separator so we can deal with Windows' complex of being unique
        local sep = package.config:sub(1, 1)
        -- Update path
        package.path = package.path .. string.format(";.%ssrc%slua%s?.lua", sep, sep, sep)
    "#,
    )
    .exec()?;

    let globals = lua.globals();

    // ===== Load NeoSH Lua scripts and functions
    // Load NeoSH extended Lua stdlib + inspect function
    let lua_neosh = lua.create_table()?;

    globals.set("neosh", lua_neosh)?;

    lua.load(NEOSH_STDLIB).set_name("neosh")?.exec()?;
    Ok(globals)
}
