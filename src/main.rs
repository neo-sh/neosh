use std::io::stdout;
use crossterm::{terminal, execute, style::Print, cursor};
use neosh::core::input;

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let mut handler = input::KeyHandler::new();
    while handler.process()? {
        execute!(
            stdout(),
            Print(&handler.buffer),
            cursor::MoveToColumn(handler.index + 1),
            cursor::Show
        )?;
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
