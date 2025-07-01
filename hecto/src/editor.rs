use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use crossterm::{event::{
        read, Event::{self}, KeyCode::{self}, KeyEvent, KeyModifiers
    }};

mod terminal;

use terminal::{Terminal, Size, Position};

pub struct Editor {
    should_quit: bool,
    current_pos: Position,
    terminal_size: Size,
}

impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false, 
            current_pos : Position{x:1, y:0},
            terminal_size: Terminal::size().unwrap_or(Size { width: 80, height: 24 }),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        match event {
            // Handle Key Presses
            Event::Key(KeyEvent { code, modifiers, ..}) => {
                match code {
                    // Quit the application
                    KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }

                    // Normal Cursor Movement
                    KeyCode::Up => {
                        self.current_pos.y = self.current_pos.y.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        let height = self.terminal_size.height.saturating_sub(1);
                        self.current_pos.y = self.current_pos.y.saturating_add(1).min(height);
                    }
                    KeyCode::Left => {
                        if self.current_pos.x != 1 {
                            self.current_pos.x = self.current_pos.x.saturating_sub(1);                            
                        }
                    }
                    KeyCode::Right => {
                        let width = self.terminal_size.width.saturating_sub(1);
                        if self.current_pos.x != width{
                            self.current_pos.x = self.current_pos.x.saturating_add(1).min(width);
                        }
                    }

                    // Page and Line Navigation
                    KeyCode::PageUp => {
                        self.current_pos.y = 0;
                    }
                    KeyCode::PageDown => {
                        self.current_pos.y = self.terminal_size.height.saturating_sub(1);
                    }
                    KeyCode::Home => {
                        self.current_pos.x = 0;
                    }
                    KeyCode::End => {
                        self.current_pos.x = self.terminal_size.width.saturating_sub(1);
                    }

                    // Ignore all other keys
                    _ => (),
                }
            }
            // Handle Window Resizing
            Event::Resize(width, height) => {
                self.terminal_size.width = (*width).into();
                self.terminal_size.height = (*height).into();
            }
            _ => (),
        }
    }

    fn draw_welcome_msg() -> Result<(), Error>{
        let mut welcome_msg = format!("{NAME} editor -- Version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_msg.len();
        // Allowed as padding doesn't have to be exactly in the middle
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        
        let space = " ".repeat(padding.saturating_sub(1));
        welcome_msg = format!("~{space}{welcome_msg}");
        welcome_msg.truncate(width);

        Terminal::print(&welcome_msg)?;
    
        Ok(())
    }   

    fn draw_empty_rows() -> Result<(), Error>{
        Terminal::print("~")?;
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(Position{x: 0, y: 0})?;
            Terminal::print("Goodbye !\r\n")?;
        } else {
            Terminal::move_cursor_to(Position{x: 0, y: 0})?;
            Self::draw_rows()?;
        }
        Terminal::move_cursor_to(self.current_pos)?;
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if current_row == height / 3{
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_rows()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
