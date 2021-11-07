mod neosh;

use std::env;
use std::path::{Path, PathBuf};

use rlua::{Error as LuaError, Lua, MultiValue};
use rustyline::{config::Configurer, config::EditMode, error::ReadlineError, Editor};

use crate::neosh::lua;

#[allow(dead_code)]
struct NeoshPaths {
    data: PathBuf,
    config: PathBuf,
}

fn main() {
    // TODO: move this into `impl NeoshPaths`
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

    Lua::new().context(|lua_context| {
        // ===== Readline setup =======
        let mut rl = Editor::<()>::new();
        // TODO: change this after establishing the initial configurations setup
        // set mode to Vi instead of the default one (Emacs)
        rl.set_edit_mode(EditMode::Vi);

        // Load previous history and ignore errors if there isn't a history file
        // TODO: move this hist file to ~/.local/share/neosh/.neosh_history later
        let _ = rl.load_history("hist.txt");

        // Load NeoSH Lua stdlib
        lua::init(lua_context);

        loop {
            let user = env!("USER");
            // Fallback to "$HOST" if running MacOS and set host to "machine" if we were unable to
            // find the hostname
            let host = option_env!("HOSTNAME").unwrap_or(option_env!("HOST").unwrap_or("machine"));
            let cwd = env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();

            // Default prompt: "[user@host /path/to/cwd] » "
            let mut prompt = format!("[{}@{} {}] » ", user, host, cwd);
            let mut line = String::new();

            loop {
                match rl.readline(&prompt) {
                    Ok(input) => line.push_str(&input),
                    // Ctrl-C, print empty line like ZSH
                    Err(ReadlineError::Interrupted) => {
                        println!("");
                        break;
                    }
                    // Ctrl-D, exit like ZSH
                    Err(ReadlineError::Eof) => return,
                    Err(_) => return,
                }

                // Separate command and arguments
                let mut cmd_parts = line.trim().split_whitespace();
                let command = cmd_parts.next().unwrap();
                let args = cmd_parts;

                // ===== Built-in commands
                // NOTE: move them later to another location (a separated module)
                match command {
                    // Exit shell
                    "exit" => {
                        // Save exit command to history before exiting shell
                        // TODO: use neosh data directory to save history once
                        // initial directories setup is done
                        rl.add_history_entry(&line);
                        return;
                    }
                    // Change cwd, see this link for more information
                    // https://unix.stackexchange.com/a/38809
                    "cd" => {
                        rl.add_history_entry(&line);
                        let home_dir = dirs::home_dir().unwrap();
                        // default to '~' as new directory if one was not provided
                        let new_dir = args
                            .peekable()
                            .peek()
                            .map_or(home_dir, |dir| PathBuf::from(dir));
                        let root = Path::new(&new_dir);
                        if let Err(err) = env::set_current_dir(&root) {
                            eprintln!("{}", err);
                        }
                        break;
                    }
                    // Interpret Lua code
                    _ => {
                        match lua_context.load(&line).eval::<MultiValue>() {
                            Ok(values) => {
                                // Save command to history and print the output
                                rl.add_history_entry(&line);
                                println!(
                                    "{}",
                                    values
                                        .iter()
                                        .map(|val| format!("{:?}", val))
                                        .collect::<Vec<_>>()
                                        .join("\t")
                                );
                                break;
                            }
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
                }
            }

            // TODO: use neosh data directory to save history once
            // initial directories setup is done
            rl.save_history("hist.txt").unwrap();
        }
    });
}
