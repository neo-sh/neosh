extern crate dirs;

use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    loop {
        // Default prompt: "[user@host /path/to/cwd] » "
        let user = env!("USER");
        let host = env!("HOSTNAME");
        let cwd = std::env::current_dir().unwrap();
        print!("[{}@{} {}] » ", user, host, cwd.display());
        // need to explicitly flush this to ensure it prints before read_line
        io::stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character
        // is interpreted as the command arguments, e.g. 'ls -a ../foo' → ['-a', '../foo']
        let mut cmd_parts = input.trim().split_whitespace();
        let command = cmd_parts.next().unwrap();
        let args = cmd_parts;

        match command {
            // Change cwd, see this link for more information
            // https://unix.stackexchange.com/a/38809
            "cd" => {
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
            }
            // Exit the shell
            "exit" => return,
            // Execute the given command
            command => {
                let child_process = Command::new(command).args(args).spawn();

                match child_process {
                    Err(err) => eprintln!("{}", err),
                    Ok(mut child_process) => {
                        // don't accept another command until this one completes
                        child_process.wait().expect("Failed to execute process");
                    }
                }
            }
        }
    }
}
