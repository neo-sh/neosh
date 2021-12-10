use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;

use tracing::{debug, error, info, trace, warn};

const NEOSH_STDLIB: &str = include_str!("../lua/neosh.lua");

fn info(_: &Lua, msg: String) -> LuaResult<()> {
    let msg = msg.as_str();
    info!(msg);
    Ok(())
}

fn warn(_: &Lua, msg: String) -> LuaResult<()> {
    let msg = msg.as_str();
    warn!(msg);
    Ok(())
}

fn error(_: &Lua, msg: String) -> LuaResult<()> {
    let msg = msg.as_str();
    error!(msg);
    Ok(())
}

fn debug(_: &Lua, msg: String) -> LuaResult<()> {
    let msg = msg.as_str();
    debug!(msg);
    Ok(())
}

fn trace(_: &Lua, msg: String) -> LuaResult<()> {
    let msg = msg.as_str();
    trace!(msg);
    Ok(())
}

// Initialize Lua globals and logging
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    // ===== Setup package path so we can require scripts
    lua.load(
        r#"
        -- Get the system separator so we can deal with Windows' complex of being unique
        local sep = package.config:sub(1, 1)
        -- Update path
        package.path = table.concat({package.path, string.format(";.%ssrc%slua%s?.lua", sep, sep, sep)})
    "#,
    )
    .exec()?;

    let globals = lua.globals();

    // ===== Load NeoSH Lua scripts and functions
    // Load NeoSH extended Lua stdlib + inspect function
    let lua_neosh = lua.create_table()?;

    globals.set("neosh", lua_neosh)?;

    lua.load(NEOSH_STDLIB).set_name("neosh")?.exec()?;

    // ===== Set logging functions
    let lua_log = lua.create_table()?;
    lua_log.set("info", lua.create_function(info)?)?;
    lua_log.set("warn", lua.create_function(warn)?)?;
    lua_log.set("error", lua.create_function(error)?)?;
    lua_log.set("debug", lua.create_function(debug)?)?;
    lua_log.set("trace", lua.create_function(trace)?)?;
    globals.set("log", lua_log)?;

    Ok(globals)
}
