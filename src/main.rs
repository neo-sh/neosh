use std::time::Duration;

use crossterm::{event::{self, Event, KeyCode, KeyEvent}, execute, terminal, style::Print};

use std::io::{Write, stdout};

struct KeyHandler;

impl KeyHandler {
    fn read(&self) -> anyhow::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event)
                }
            }
        }
    }

    fn process(&self) -> anyhow::Result<bool> {
        match self.read()? {
            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
            } => execute!(stdout(), Print(ch))?,
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: event::KeyModifiers::NONE,
            } => println!(),
            _ => (),
        };

        return Ok(true)
    }
}

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::Purge))?;
    while KeyHandler.process()? {}

    terminal::disable_raw_mode()?;

    Ok(())
}
