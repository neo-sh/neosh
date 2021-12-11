use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;

use tracing::{debug, error, info, trace, warn};

const NEOSH_STDLIB: &str = include_str!("../lua/neosh.lua");

macro_rules! import_log {
    ($level:ident) => {
        fn $level(_: &Lua, msg: String) -> LuaResult<()> {
            let msg = msg.as_str();
            $level!("{}", msg);
            Ok(())
        }
    }
}

import_log!(info);
import_log!(warn);
import_log!(error);
import_log!(debug);
import_log!(trace);

// Initialize Lua globals and logging
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    debug!("Initializing NeoSH Lua stdlib");
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
    debug!("Set up package path");

    let globals = lua.globals();

    // ===== Load NeoSH Lua scripts and functions
    // Load NeoSH extended Lua stdlib + inspect function
    let lua_neosh = lua.create_table()?;

    globals.set("neosh", lua_neosh)?;

    lua.load(NEOSH_STDLIB).set_name("neosh")?.exec()?;
    debug!("Loaded NeoSH Lua stdlib");

    // ===== Set logging functions
    let lua_log = lua.create_table()?;
    lua_log.set("info", lua.create_function(info)?)?;
    lua_log.set("warn", lua.create_function(warn)?)?;
    lua_log.set("error", lua.create_function(error)?)?;
    lua_log.set("debug", lua.create_function(debug)?)?;
    lua_log.set("trace", lua.create_function(trace)?)?;
    globals.set("log", lua_log)?;

    debug!("Set up logging for Lua");

    Ok(globals)
}
