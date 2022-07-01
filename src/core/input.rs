use std::{io::Stdout, time::Duration};

use bstr::BString;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use miette::IntoDiagnostic;

use std::io::stdout;

pub struct KeyHandler {
    pub buffer: BString,
    pub index: usize,
    stdout: Stdout,
    pub execute: bool,
    pub incomplete: BString,
    pub prompt: BString,
}

impl Default for KeyHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyHandler {
    pub fn new() -> Self {
        Self {
            buffer: BString::default(),
            index: 0,
            stdout: stdout(),
            execute: false,
            incomplete: BString::default(),
            prompt: BString::default(),
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
                let mut buf = [0; 4];
                // SAFETY: We don't treat the slice returned by `encode_utf8` as as a
                // str again, so this can't invoke UB.
                let s = unsafe { ch.encode_utf8(&mut buf).as_bytes_mut() };
                s.reverse();
                for c in s {
                    self.buffer.insert(self.index, *c);
                }
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
                self.buffer.extend_from_slice(b"    ");
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
            // HOME
            KeyEvent {
                code: KeyCode::Home,
                ..
            } => {
                self.index = 0;
            }
            // END
            KeyEvent {
                code: KeyCode::End, ..
            } => {
                self.index = self.buffer.len();
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
