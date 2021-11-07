use super::lua;
use rlua::Table; // We'll soon expose several utility functions in this module

struct Prompt<'a> {
    components: Table<'a>,
}

pub fn create() {}
