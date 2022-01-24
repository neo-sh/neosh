//! All built-in commands available in neosh
use crate::log::utils::command;
use tracing::error;

use std::{
    env,
    path::{Path, PathBuf},
    str::SplitWhitespace,
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
pub fn cd(args: SplitWhitespace) {
    command("cd");
    let home_dir = dirs::home_dir().unwrap();

    let next_dir = args.peekable().peek().map_or(home_dir, PathBuf::from);
    let next_dir = Path::new(&next_dir);

    if let Err(err) = env::set_current_dir(next_dir) {
        error!("Failed to change directory: {}", err);
    }
}

/// Print current working directory
// NOTE: I am not importing cwd from main.rs because we might change structure (Shift)
pub fn pwd() {
    command("pwd");
    println!(
        "{}",
        env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );
}

/// Print input
// TODO: make it use stdin
pub fn echo(args: SplitWhitespace) {
    command("echo");

    println!("{}", { args.collect::<Vec<&str>>().join(" ") })
}
