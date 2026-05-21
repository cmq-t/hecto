use clap::Parser;
use crossterm::event::{Event, Event::Key, KeyCode, KeyEvent, KeyModifiers, read};
use std::cmp::min;
use terminal::{Position, Terminal};
use view::View;

mod terminal;
mod view;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    filename: Option<String>,
}

#[derive(Default)]
pub struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn to_position(&self) -> Position {
        Position {
            col: self.x,
            row: self.y,
        }
    }
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.handle_args();
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_cursor(*code)?;
                }
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(self.cursor.to_position())?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn move_cursor(&mut self, code: KeyCode) -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        match code {
            KeyCode::Up => self.cursor.y = self.cursor.y.saturating_sub(1),
            KeyCode::Down => self.cursor.y = min(self.cursor.y.saturating_add(1), size.height - 1),
            KeyCode::Left => self.cursor.x = self.cursor.x.saturating_sub(1),
            KeyCode::Right => self.cursor.x = min(self.cursor.x.saturating_add(1), size.width - 1),
            KeyCode::PageUp => self.cursor.y = 0,
            KeyCode::PageDown => self.cursor.y = size.height - 1,
            KeyCode::Home => self.cursor.x = 0,
            KeyCode::End => self.cursor.x = size.width - 1,
            // TODO, someday
            _ => (),
        }
        Ok(())
    }

    fn handle_args(&mut self) {
        let args = Args::parse();
        if let Some(filename) = args.filename {
            self.view.load(filename);
        }
    }
}
