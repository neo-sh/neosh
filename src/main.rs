use std::io::stdout;
use crossterm::{terminal, execute, style::Print, cursor};
use mlua::{Error as LuaError, Lua, MultiValue};
use neosh::core::{self, commands, input, fs, lua as nlua};
use tracing::{debug, info, error};

fn init() -> fs::NeoshPaths {
    let mut data = dirs::data_dir().unwrap();
    let mut cache = dirs::cache_dir().unwrap();
    let mut config = dirs::config_dir().unwrap();

    data.push("neosh");
    cache.push("neosh");
    config.push("neosh");

    let neosh_paths = fs::NeoshPaths {
        data,
        cache,
        config,
    };

    if let Err(err) = &neosh_paths.create_neosh_dirs() {
        eprintln!("Failed to create NeoSH core directories: {err}");
    };

    std::env::set_var("NEOSH_VERSION", core::VERSION);

    neosh_paths
}

fn main() -> anyhow::Result<()> {
    let neosh_paths = init();
    let _log_guard = neosh::log::setup(&neosh_paths.data);
    let lua = Lua::new();
    info!("Set up Lua instance");

    nlua::init(&lua).unwrap();
    info!("Loaded NeoSH Lua stdlib");

    let user = whoami::username();
    let host = whoami::hostname();
    info!("Fetched user data: {}@{}", user, host);

    terminal::enable_raw_mode()?;
    debug!("Entered raw mode");

    let mut handler = input::KeyHandler::new();
    while handler.process()? {
        if handler.execute {
            debug!("Executing buffer");
            let prev = handler.incomplete;
            handler.incomplete = String::new();

            let mut args = handler.buffer.trim().split_whitespace();
            let command = args.next().unwrap_or("");
            match command {
                "exit" => {
                    commands::exit();
                    break;
                },
                "cd" => {
                    commands::cd(args);
                },
                "pwd" => {
                    commands::pwd();
                },
                "echo" => {
                    commands::echo(args);
                },
                "" => (),
                _ => match lua.load(&format!("{prev}{}", &handler.buffer)).eval::<MultiValue>() {
                    Ok(values) => {
                        println!("{}",
                                 values
                                 .iter()
                                 .map(|val| format!("{val:?}"))
                                 .collect::<Vec<_>>()
                                 .join("\t")
                                 );
                    },
                    Err(LuaError::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        handler.incomplete = format!("{prev}{}\n", handler.buffer);
                    },
                    Err(err) => {
                        error!("Unrecognised Lua error: {}", err);
                    }
                }
            }

            handler.buffer = String::new();
            execute!(stdout(), cursor::Show)?;
        } else {
            execute!(
                stdout(),
                Print(&handler.buffer),
                cursor::MoveToColumn(handler.index + 1),
                cursor::Show
                )?;
        }
    }

    execute!(stdout(), cursor::Show)?;
    terminal::disable_raw_mode()?;
    debug!("Exited from raw mode");

    Ok(())
}
