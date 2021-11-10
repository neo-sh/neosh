use std::env;
use std::path::{Path, PathBuf};

use mlua::{Error as LuaError, Lua, MultiValue};
use rustyline::{config::Configurer, config::EditMode, error::ReadlineError, Editor};

use neosh::core::{self, fs, lua as nlua};

/// Run pre-launch tasks. Create NeoSH directories and expose environment variables
fn init() -> fs::NeoshPaths {
    // Declare NeoSH directories, e.g. '~/.cache/neosh'
    let mut data_dir = dirs::data_dir().unwrap();
    data_dir.push("neosh");
    let mut cache_dir = dirs::cache_dir().unwrap();
    cache_dir.push("neosh");
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("neosh");

    let neosh_paths = fs::NeoshPaths {
        data: data_dir,
        cache: cache_dir,
        config: config_dir,
    };

    // Create core NeoSH directories, e.g. '~/.local/share/neosh'
    match &neosh_paths.create_neosh_dirs() {
        Ok(_) => (),
        Err(err) => eprintln!("Failed to create NeoSH core directories: {}", err),
    };

    // Expose NeoSH version as an environment variable
    env::set_var("NEOSH_VERSION", core::VERSION);

    neosh_paths
}

fn main() {
    // Run initial tasks and get the NeoSH paths
    let neosh_paths = init();

    // ===== Readline setup ======
    let mut readline = Editor::<()>::new();
    // NOTE: change this after establishing the initial configurations setup
    // set mode to Vi instead of the default one (Emacs)
    readline.set_edit_mode(EditMode::Vi);

    // Setup history file path
    let mut history_path = PathBuf::from(&neosh_paths.data);
    history_path.push(".neosh_history");
    let history_path = history_path.into_os_string().into_string().unwrap();

    // Load previous history and ignore errors if there isn't a history file
    let _ = readline.load_history(&history_path);

    // ===== Lua setup ===========
    let lua = Lua::new();

    // Load NeoSH Lua stdlib
    nlua::init(&lua).unwrap();

    loop {
        let user = whoami::username();
        let host = whoami::hostname();
        let cwd = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        // Default prompt: "[user@host /path/to/cwd] » "
        let mut prompt = format!("[{}@{} {}] » ", user, host, cwd);
        let mut line = String::new();

        loop {
            match readline.readline(&prompt) {
                Ok(input) => line.push_str(&input),
                // Ctrl-C, print empty line like ZSH
                Err(ReadlineError::Interrupted) => {
                    println!();
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
                    readline.add_history_entry(&line);
                    return;
                }
                // Change cwd, see this link for more information
                // https://unix.stackexchange.com/a/38809
                "cd" => {
                    readline.add_history_entry(&line);
                    let home_dir = dirs::home_dir().unwrap();
                    // default to '~' as new directory if one was not provided
                    let new_dir = args
                        .peekable()
                        .peek()
                        .map_or(home_dir, PathBuf::from);
                    let root = Path::new(&new_dir);
                    if let Err(err) = env::set_current_dir(&root) {
                        eprintln!("{}", err);
                    }
                    break;
                }
                // Interpret Lua code
                _ => match lua.load(&line).eval::<MultiValue>() {
                    Ok(values) => {
                        // Save command to history and print the output
                        readline.add_history_entry(&line);
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
                        line.push('\n'); // separate input lines
                        prompt = "> ".to_string();
                    }
                    Err(err) => {
                        eprintln!("error: {}", err);
                        break;
                    }
                }
            }
        }

        // TODO: Make an option to save history after every command instead of having to wait until
        // the user exits the shell
        readline.save_history(&history_path).unwrap();
    }
}
