use super::terminal::{Position, Size, Terminal};
use buffer::Buffer;

mod buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const FORMAT: &str = " editor —— version ";

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::move_caret_to(Position {
                col: 0,
                row: current_row,
            })?;
            Terminal::clear_line()?;
            if current_row == height / 3 && self.buffer.is_empty() {
                Self::draw_welcome_message()?;
            } else if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    fn render_buffer(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::move_caret_to(Position {
                col: 0,
                row: current_row,
            })?;
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    fn render_welcome_screen() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::move_caret_to(Position {
                col: 0,
                row: current_row,
            })?;
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let string_length = NAME.len() + FORMAT.len() + VERSION.len();
        let Size { width, .. } = Terminal::size()?;
        let padding = " ".repeat(width.saturating_sub(string_length) / 2 - 1);
        let message = format!("~{padding}{NAME}{FORMAT}{VERSION}");
        Terminal::print(&message)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, filename: String) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }
}
