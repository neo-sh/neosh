use crate::core::commands;
use miette::miette;
use miette::IntoDiagnostic;
use miette::WrapErr;
use mlua::Error as LuaError;
use mlua::Lua;
use mlua::MultiValue;
use std::io::stdout;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style::Print;

use std::str::SplitWhitespace;

pub enum ExecutorType {
    Internal(InternalExecutor),
    External(&String),
}

impl ExecutorType {
    fn get_args(&self) -> SplitWhitespace {
        match self {
            Self::Internal(int) => int.buffer.trim().split_whitespace(),
            Self::External(str) => str.trim().split_whitespace(),
        }
    }
    fn build_lua_string(&self) -> String {
        match self {
            Self::Internal(int) => format!("{}{}", int.prev, int.buffer),
            Self::External(str) => str.to_string(),
        }
    }
}

pub struct InternalExecutor {
    incomplete: &mut String,
    prompt: &mut String,
    buffer: &mut String,
    prev: String,
}

impl InternalExecutor {
    pub fn new(incomplete: &mut String, prompt: &mut String, buffer: &mut String) -> Self {
        let prev = incomplete.to_string();
        *incomplete = String::new();
        Self {
            incomplete,
            prompt,
            buffer,
            prev,
        }
    }
}

pub struct Executor {
    variant: ExecutorType,
    args: SplitWhitespace,
    command: String,
    lua: &Lua,
    prev: String,
}

impl Executor {
    pub fn new(variant: ExecutorType, lua: &Lua) -> Self {
        let mut args = variant.get_args();
        let command = args.next().unwrap_or("");
        let prev = String::new();
        Self {
            variant,
            args,
            command,
            lua,
            prev,
        }
    }
    fn debug(&self) {
        match self.variant {
            ExecutorType::Internal(_) => tracing::debug!("Executing buffer"),
            ExecutorType::External(_) => tracing::debug!("Executing external input"),
        }
    }
    fn handle(&self) -> miette::Result<()> {
        match self.command {
            "exit" => {
                commands::exit();
            }
            "cd" => commands::cd()?,
            "pwd" => commands::pwd()?,
            "echo" => commands::echo(self.args),
            "" => (),
            _ => self.lua(),
        }

        Ok(())
    }

    fn lua(&self) -> miette::Result<()> {
        match self
            .lua
            .load(&self.variant.build_lua_string())
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
            Err(err) => match self.variant {
                ExecutorType::Internal(int) => match err {
                    LuaError::SyntaxError {
                        incomplete_input: true,
                        ..
                    } => {
                        *int.incomplete = format!("{}{}\n", int.prev, int.buffer);
                    }
                    _ => {
                        crossterm::terminal::disable_raw_mode().into_diagnostic()?;
                        println!("{:?}", miette!(err).wrap_err("Lua Error"));
                        crossterm::terminal::enable_raw_mode().into_diagnostic()?;
                    }
                },
                // TODO: remove duplicate code
                ExecutorType::External(_) => {
                    crossterm::terminal::disable_raw_mode().into_diagnostic()?;
                    println!("{:?}", miette!(err).wrap_err("Lua Error"));
                    crossterm::terminal::enable_raw_mode().into_diagnostic()?;
                }
            },
        }
        Ok(())
    }

    fn reset(&self) -> miette::Result<()> {
        if let ExecutorType::Internal(int) = self.variant {
            *int.buffer = String::new();
            execute!(
                stdout(),
                cursor::MoveToColumn(0),
                Print(handler.prompt),
                cursor::Show
            )
            .into_diagnostic()?;
        }
        Ok(())
    }

    pub fn execute(&mut self) -> miette::Result<()> {
        self.debug();
        self.handle()?;
        self.reset()?;
        Ok(())
    }
}
