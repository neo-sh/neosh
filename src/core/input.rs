use std::{io::Stdout, time::Duration};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use miette::IntoDiagnostic;

use std::io::stdout;

pub struct KeyHandler {
    pub buffer: String,
    pub index: u16,
    stdout: Stdout,
    pub execute: bool,
    pub incomplete: String,
    pub prompt: String,
}

impl KeyHandler {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            index: 0,
            stdout: stdout(),
            execute: false,
            incomplete: String::new(),
            prompt: String::new(),
        }
    }

    fn read(&self) -> miette::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500)).into_diagnostic()? {
                if let Event::Key(event) = event::read().into_diagnostic()? {
                    return Ok(event);
                }
            }
        }
    }

    pub fn process(&mut self) -> miette::Result<bool> {
        self.execute = false;
        match self.read()? {
            // exit
            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            // Char
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
            } => {
                self.buffer.insert(self.index as usize, ch);
                self.index += 1;
            }
            // BackSpace
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                if self.index != 0 {
                    self.index -= 1;
                    self.buffer.remove(self.index as usize);
                }
                execute!(self.stdout, cursor::MoveLeft(1)).into_diagnostic()?;
            }
            // Del
            KeyEvent {
                code: KeyCode::Delete,
                ..
            } => {
                if (self.index as usize) < self.buffer.len() {
                    self.buffer.remove(self.index as usize);
                }
            }
            // Tab
            KeyEvent {
                code: KeyCode::Tab, ..
            } => {
                self.index += 4;
                self.buffer.push_str(&" ".repeat(4));
            }
            // CR
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
            } => {
                self.index = 0;
                self.execute = true;
                println!();
            }
            // Arrows
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE,
            } => {
                if self.index != 0 {
                    self.index -= 1
                }
                execute!(self.stdout, cursor::MoveLeft(1)).into_diagnostic()?;
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE,
            } => {
                if (self.index as usize) < self.buffer.len() {
                    self.index += 1
                }
                execute!(self.stdout, cursor::MoveRight(1)).into_diagnostic()?;
            }
            _ => (),
        };

        execute!(
            self.stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveToColumn(1),
        )
        .into_diagnostic()?;

        Ok(true)
    }
}
