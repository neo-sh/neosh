//! All built-in commands available in neosh
use crate::log::utils::command;
use miette::{miette, IntoDiagnostic, WrapErr};
use tracing::error;

use std::{
    borrow::Cow,
    env,
    io::{stdout, Write},
    path::Path,
};

/// Exit shell
///
/// Actually, does nothing but saving last cmd in history
pub fn exit() {
    command("exit");
}

/// Change current working directory
// https://unix.stackexchange.com/a/38809
// TODO: check if path exists
pub fn cd<'a>(mut args: impl Iterator<Item = &'a [u8]>) -> miette::Result<()> {
    command("cd");
    let next_dir = if let Some(next_dir) = args.next() {
        Cow::Borrowed(Path::new(
            std::str::from_utf8(next_dir)
                .into_diagnostic()
                .wrap_err("Path is invalid UTF-8")?,
        ))
    } else {
        Cow::Owned(dirs::home_dir().ok_or_else(|| miette!("Failed to get home directory"))?)
    };

    if let Err(err) = env::set_current_dir(next_dir) {
        error!("Failed to change directory: {}", err);
    }

    Ok(())
}

/// Print current working directory
// NOTE: I am not importing cwd from main.rs because we might change structure (Shift)
pub fn pwd() -> miette::Result<()> {
    command("pwd");
    println!(
        "{}",
        env::current_dir()
            .into_diagnostic()
            .wrap_err("Failed to get current directory")?
            .to_string_lossy(),
    );

    Ok(())
}

/// Print input
// TODO: make it use stdin
pub fn echo<'a>(args: impl Iterator<Item = &'a [u8]>) -> miette::Result<()> {
    command("echo");

    let mut stdout = stdout();

    let mut first = true;
    for a in args {
        if !first {
            stdout.write_all(b" ").into_diagnostic()?;
        }

        stdout.write_all(a).into_diagnostic()?;
        first = false;
    }

    stdout.write_all(b"\n").into_diagnostic()?;

    Ok(())
}
