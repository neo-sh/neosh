//! NeoSH Lua manager

use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;

use tracing::{debug, error, info, trace, warn};

use crate::core::{self, commands};

const NEOSH_STDLIB: &[u8] = include_bytes!("../lua/neosh.lua");
const NEOSH_INSPECT: &[u8] = include_bytes!("../lua/inspect.lua");

/// Generate log functions that can be bridged into Lua
macro_rules! import_log {
    ($level:ident) => {
        fn $level(_: &Lua, msg: String) -> LuaResult<()> {
            let msg = msg.as_str();
            $level!("{}", msg);
            Ok(())
        }
    };
}

import_log!(info);
import_log!(warn);
import_log!(error);
import_log!(debug);
import_log!(trace);

/// Initialize Lua globals and logging
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    debug!("Initializing NeoSH Lua stdlib");
    let globals = lua.globals();

    // ===== Load NeoSH Lua scripts and functions
    let lua_neosh = lua.create_table()?;

    // Expose NeoSH version as a "constant"
    lua_neosh.set("VERSION", core::VERSION)?;

    // Expose built-in NeoSH commands to Lua side (except exit command because it does nothing)
    let cd_fn = lua.create_function(|_, path: Option<String>| {
        // If no path was passed then fallback to $HOME
        let _ = commands::cd(
            path.unwrap_or_else(|| "".to_string())
                .split_whitespace()
                .map(str::as_bytes),
        );
        Ok(())
    })?;
    lua_neosh.set("cd", cd_fn)?;

    let pwd_fn = lua.create_function(|_, ()| {
        commands::pwd().unwrap();
        Ok(())
    })?;
    lua_neosh.set("pwd", pwd_fn)?;

    // Set logging functions and expose them as 'neosh.log'
    let lua_log = lua.create_table()?;
    lua_log.set("info", lua.create_function(info)?)?;
    lua_log.set("warn", lua.create_function(warn)?)?;
    lua_log.set("error", lua.create_function(error)?)?;
    lua_log.set("debug", lua.create_function(debug)?)?;
    lua_log.set("trace", lua.create_function(trace)?)?;
    lua_neosh.set("log", lua_log)?;

    debug!("Set up logging for Neosh Lua stdlib");

    globals.set("neosh", lua_neosh)?;

    // Load NeoSH stdlib
    lua.load(NEOSH_STDLIB).set_name("neosh")?.exec()?;
    lua.load(NEOSH_INSPECT).set_name("inspect")?.exec()?;

    debug!("Loaded NeoSH Lua stdlib");

    Ok(globals)
}
