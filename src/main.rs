// TODO: finish the initial Lua file that will load our stdlib (lua.rs) and discover how to load
// local modules, it seems like this is a pain like Python modules?
// mod lua;
use std::env;
use std::path::PathBuf;

// use dirs;
use rlua::{Error as LuaError, Lua, MultiValue};
use rustyline::Editor;
use rustyline::{config::Configurer, config::EditMode};
use rustyline::error::ReadlineError;

#[allow(dead_code)]
struct NeoshPaths {
    data: PathBuf,
    config: PathBuf,
}

const NEOSH_STDLIB: &str = include_str!("lua/neostd.lua");

fn main() {
    // Set up NEOSH paths, we are doing it this way to allow cross-platform compatibility
    // TODO: finish the initial implementation to use data directory to save history later
    /* let mut neosh_data_dir = dirs::data_dir().unwrap();
    neosh_data_dir.push("neosh");
    let mut neosh_config_dir = dirs::config_dir().unwrap();
    neosh_config_dir.push("neosh");
    let _paths = NeoshPaths {
        data: neosh_data_dir,
        config: neosh_config_dir,
    }; */

    Lua::new().context(|lua_ctx| {
        // ===== Readline setup =======
        let mut rl = Editor::<()>::new();
        // TODO: change this after establishing the initial configurations setup
        // set mode to Vi instead of the default one (Emacs)
        rl.set_edit_mode(EditMode::Vi);

        // Load previous history and ignore errors if there isn't a history file
        // TODO: move this hist file to ~/.local/share/neosh/.neosh_history later
        let _ = rl.load_history("hist.txt");

        // ===== Lua ==================
        // Setup package path so we can require scripts
        lua_ctx.load(r#"
            -- Get the system separator so we can deal with Windows' complex of being unique
            local sep = package.config:sub(1, 1)
            -- Update path
            package.path = package.path .. string.format(";.%ssrc%slua%s?.lua", sep, sep, sep)
        "#).exec().unwrap();

        // Load NeoSH Lua scripts
        // lua::init();
        let globals = lua_ctx.globals();
        let lua_neosh = lua_ctx.create_table().unwrap();
        globals.set("neostd", lua_neosh).unwrap();
        // Load NeoSH extended Lua stdlib + inspect function
        lua_ctx.load(NEOSH_STDLIB).set_name("neostd").unwrap().exec().unwrap();
        // lua_ctx.load("neostd.inspect = require('inspect')").exec().unwrap();

        loop {
            // Default prompt: "[user@host /path/to/cwd] » "
            let user = env!("USER");
            let host = env!("HOSTNAME");
            let cwd = std::env::current_dir().unwrap();

            let mut prompt = format!("[{}@{} {}] » ", user, host, cwd.display());
            let mut line = String::new();

            loop {
                match rl.readline(&prompt) {
                    Ok(input) => line.push_str(&input),
                    // Ctrl-C, print empty line like ZSH
                    Err(ReadlineError::Interrupted) => {
                        println!("");
                        break;
                    },
                    // Ctrl-D, exit like ZSH
                    Err(ReadlineError::Eof) => {
                        return
                    },
                    Err(_) => return,
                }

                // NOTE: maybe this could be done in other way?
                if &line == "exit" {
                    // Save exit command to history before exiting shell
                    // TODO: use neosh data directory to save history once
                    // initial directories setup is done
                    rl.add_history_entry("exit");
                    lua_ctx.load("neostd.exit()").exec().unwrap();
                }

                match lua_ctx.load(&line).eval::<MultiValue>() {
                    Ok(values) => {
                        rl.add_history_entry(line);
                        let output = format!(
                            "{}",
                            values
                                .iter()
                                .map(|val| format!("{:?}", val))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                        println!("{}", output);
                        break;
                    },
                    Err(LuaError::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        // continue reading input and append it to `line`
                        line.push_str("\n"); // separate input lines
                        prompt = "> ".to_string();
                    }
                    Err(err) => {
                        eprintln!("error: {}", err);
                        break;
                    }
                }
            }
            // TODO: use neosh data directory to save history once
            // initial directories setup is done
            rl.save_history("hist.txt").unwrap();
        }
    });
}
