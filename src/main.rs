use std::{io::Stdout, time::Duration};

use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent}, execute, queue, style::Print, terminal};

use std::io::{Write, stdout};

struct KeyHandler {
    buffer: String,
    index: u16,
    stdout: Stdout,
}

impl KeyHandler {
    fn new() -> Self {
        Self { buffer: String::new(), index: 0, stdout: stdout() }
    }

    fn read(&self) -> anyhow::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event)
                }
            }
        }
    }

    fn process(&mut self) -> anyhow::Result<bool> {
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
            },
            // BackSpace
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                if self.index != 0 {
                    self.index -= 1;
                    self.buffer.remove(self.index as usize);
                }
                execute!(self.stdout, cursor::MoveLeft(1))?;
            },
            // Del
            KeyEvent {
                code: KeyCode::Delete,
                ..
            } => {
                if (self.index as usize) < self.buffer.len() {
                    self.buffer.remove(self.index as usize);
                }
            },
            // CR
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
            } => { 
                self.buffer = String::new();
                self.index = 0;
                println!();
            },
            // Arrows
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE,
            } => {
                if self.index != 0 { self.index -= 1 }
                execute!(self.stdout, cursor::MoveLeft(1))?;
            },
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE,
            } => {
                if (self.index as usize) < self.buffer.len() { self.index += 1 }
                execute!(self.stdout, cursor::MoveRight(1))?;
            },
            _ => (),
        };

        execute!(self.stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveToColumn(1),
        )?;

        return Ok(true)
    }
}

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let mut handler = KeyHandler::new();
    while handler.process()? {
        execute!(stdout(),
            Print(&handler.buffer),
            cursor::MoveToColumn(handler.index + 1),
            cursor::Show
        )?;
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
