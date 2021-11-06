use dirs;
use rlua::Lua, MultiValue;

const NEOSH_STDLIB: &str = include_str!("lua/neostd.lua");
const NEOSH_CONFIG: &str =
    include_str!(format!("{}/neosh/config.lua", dirs::config_dir().unwrap()));

// Initialize Lua globals
pub fn init(lua: &Lua) {
    let globals = lua.globals();

    // Load NeoSH extended Lua stdlib
    let lua_neostd = lua.create_table().unwrap();
    globals.set("neostd", lua_neostd).unwrap();
    lua.load(NEOSH_STDLIB).set_name("neostd").unwrap().exec().unwrap();
}
