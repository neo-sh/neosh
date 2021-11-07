use rlua::Context;

const NEOSH_STDLIB: &str = include_str!("../lua/neosh.lua");

// Initialize Lua globals
pub fn init(lua: Context) {
    // ===== Setup package path so we can require scripts
    lua.load(
        r#"
        -- Get the system separator so we can deal with Windows' complex of being unique
        local sep = package.config:sub(1, 1)
        -- Update path
        package.path = package.path .. string.format(";.%ssrc%slua%s?.lua", sep, sep, sep)
    "#,
    )
    .exec()
    .unwrap();

    let globals = lua.globals();

    // ===== Load NeoSH Lua scripts
    // Load NeoSH extended Lua stdlib + inspect function
    let lua_neostd = lua.create_table().unwrap();

    globals.set("neosh", lua_neostd).unwrap();

    lua.load(NEOSH_STDLIB)
        .set_name("neosh")
        .unwrap()
        .exec()
        .unwrap();
}
