use bstr::ByteSlice;
use crossterm::{cursor, execute, style::Print, terminal};
use miette::{miette, IntoDiagnostic, WrapErr};
use mlua::{Error as LuaError, Lua, MultiValue};
use neosh::core::{self, commands, fs, input, lua as nlua};
use std::io::{stdout, Write};
use tracing::{debug, info};

fn init() -> miette::Result<fs::NeoshPaths> {
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

    std::env::set_var("NEOSH_VERSION", core::VERSION);

    Ok(neosh_paths)
}

fn main() -> miette::Result<()> {
    // show panics with miette's fancy messages
    miette::set_panic_hook();
    let neosh_paths = init();
    let _log_guard = neosh::log::setup(&neosh_paths.wrap_err("Error getting directories")?.data);
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
    handler.prompt = format!("{user}@{host}$ ").into();
    stdout().write_all(&handler.prompt).into_diagnostic()?;
    while handler.process()? {
        handler.prompt = format!("{user}@{host}$ ").into();
        if handler.execute {
            debug!("Executing buffer");

            let prev = handler.incomplete.clone();
            handler.incomplete.clear();

            let mut args = handler
                .buffer
                .trim()
                .split(|c| (*c as char).is_whitespace());
            let command = args.next().unwrap_or(&[]);
            match command {
                b"exit" => {
                    commands::exit();
                    break;
                }
                b"cd" => {
                    commands::cd(args)?;
                }
                b"pwd" => {
                    commands::pwd()?;
                }
                b"echo" => {
                    commands::echo(args)?;
                }
                b"" => (),
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
                    }
                    Err(LuaError::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        handler.incomplete = format!("{prev}{}\n", handler.buffer).into();
                    }
                    Err(err) => {
                        terminal::disable_raw_mode().into_diagnostic()?;
                        println!("{:?}", miette!(err).wrap_err("Lua Error"));
                        terminal::enable_raw_mode().into_diagnostic()?;
                    }
                },
            }

            handler.buffer.clear();
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
                cursor::MoveToColumn((handler.index + handler.prompt.len() + 1) as u16),
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
