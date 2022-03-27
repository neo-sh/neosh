use crossterm::{cursor, execute, style::Print, terminal};
use miette::{miette, IntoDiagnostic, WrapErr};
use mlua::{Error as LuaError, Lua, MultiValue};
use neosh::core::{self, commands, executor, fs, input, lua as nlua};
use std::io::stdout;
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
    handler.prompt = format!("{user}@{host}$ ");
    execute!(stdout(), Print(&handler.prompt)).into_diagnostic()?;
    while handler.process()? {
        handler.prompt = format!("{user}@{host}$ ");
        if handler.execute {
            executor::Executor::new(
                executor::ExecutorType::Internal(executor::InternalExecutor::new(
                    &mut handler.incomplete,
                    &mut handler.prompt,
                    &mut handler.buffer,
                )),
                &lua,
            )
            .execute?;
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
