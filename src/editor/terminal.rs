use crossterm::Command;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::Write;
use std::io::stdout;

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), std::io::Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        let width = width as usize;
        let height = height as usize;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn print(s: &str) -> Result<(), std::io::Error> {
        Self::queue_command(Print(s))?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), std::io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), std::io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
