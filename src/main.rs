use crossterm::{cursor, execute, style::Print, terminal};
use miette::{miette, IntoDiagnostic, WrapErr};
use mlua::{Error as LuaError, Lua, MultiValue};
use neosh::core::{self, commands, fs, history, input, lua as nlua};
use std::io::stdout;
use tracing::{debug, info};

fn init() -> miette::Result<fs::NeoshPaths> {
    // Set up NeoSH directories
    let mut data = dirs::data_dir().ok_or_else(|| miette!("Couldn't get data directory"))?;
    let mut cache = dirs::cache_dir().ok_or_else(|| miette!("Couldn't get cache directory"))?;
    let mut config = dirs::config_dir().ok_or_else(|| miette!("Couldn't get config directory"))?;
    data.push("neosh");
    cache.push("neosh");
    config.push("neosh");

    let neosh_paths = fs::NeoshPaths {
        data,
        cache,
        config,
    };

    neosh_paths
        .create_neosh_dirs()
        .into_diagnostic()
        .wrap_err("Failed to create NeoSH core directories")?;

    // Set NEOSH_VERSION environment variable
    std::env::set_var("NEOSH_VERSION", core::VERSION);

    Ok(neosh_paths)
}

fn main() -> miette::Result<()> {
    // show panics with miette's fancy messages
    miette::set_panic_hook();
    let neosh_paths = init();
    let neosh_data = &neosh_paths.wrap_err("Error getting directories")?.data;
    let _log_guard = neosh::log::setup(neosh_data);

    // Set history file path and create history file if not exists
    let mut history_path = neosh_data.clone();
    history_path.push("neosh_history");
    let history = history::NeoshHistory { path: history_path };
    history.init()?;

    let lua = Lua::new();
    info!("Set up Lua instance");

    nlua::init(&lua)
        .into_diagnostic()
        .wrap_err("Failed to initialize lua manager")?;
    info!("Loaded NeoSH Lua stdlib");

    let user = whoami::username();
    let host = whoami::hostname();
    info!("Fetched user data: {}@{}", user, host);

    terminal::enable_raw_mode().into_diagnostic()?;
    debug!("Entered raw mode");

    let mut handler = input::KeyHandler::new();
    handler.prompt = format!("{user}@{host}$ ");
    execute!(stdout(), Print(&handler.prompt)).into_diagnostic()?;
    while handler.process()? {
        handler.prompt = format!("{user}@{host}$ ");
        if handler.execute {
            debug!("Executing buffer");

            let prev = handler.incomplete;
            handler.incomplete = String::new();

            let mut args = handler.buffer.trim().split_whitespace();
            let command = args.next().unwrap_or("");
            match command {
                "exit" => {
                    history.save("0:exit")?;
                    commands::exit();
                    break;
                }
                "cd" => {
                    let path = args.clone();
                    let exit_code = commands::cd(args)?;
                    let cmd = format!(
                        "{}: {} {}",
                        exit_code,
                        "cd",
                        path.collect::<Vec<&str>>().join(" ")
                    );
                    history.save(&cmd)?;
                }
                "pwd" => {
                    commands::pwd()?;
                    history.save("0:pwd")?;
                }
                "echo" => {
                    let echo_args = args.clone();
                    commands::echo(args);
                    let cmd = format!(
                        "0:{} {}",
                        "echo",
                        echo_args.collect::<Vec<&str>>().join(" ")
                    );
                    history.save(&cmd)?;
                }
                "" => (),
                _ => match lua
                    .load(&format!("{prev}{}", &handler.buffer))
                    .eval::<MultiValue>()
                {
                    Ok(values) => {
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|val| format!("{val:?}"))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                        let cmd = format!("0:{prev}{}", &handler.buffer);
                        history.save(&cmd)?;
                    }
                    Err(LuaError::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        handler.incomplete = format!("{prev}{}\n", handler.buffer);
                    }
                    Err(err) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        println!("{:?}", miette!(err).wrap_err("Lua Error"));
                        let cmd = format!("1:{prev}{}", &handler.buffer);
                        history.save(&cmd)?;
                        terminal::enable_raw_mode().into_diagnostic()?;
                    }
                },
            }

            handler.buffer = String::new();
            execute!(
                stdout(),
                cursor::MoveToColumn(0),
                Print(&handler.prompt),
                cursor::Show
            )
            .into_diagnostic()?;
        } else {
            execute!(
                stdout(),
                Print(&handler.prompt),
                Print(&handler.buffer),
                cursor::MoveToColumn(handler.index + handler.prompt.len() as u16 + 1),
                cursor::Show
            )
            .into_diagnostic()?;
        }
    }

    execute!(stdout(), cursor::Show).into_diagnostic()?;
    terminal::disable_raw_mode().into_diagnostic()?;
    debug!("Exited from raw mode");

    Ok(())
}
