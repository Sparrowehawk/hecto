use std::{env::{self}, io:: Error, panic::{set_hook, take_hook}};

use crossterm::{event::{
        read, Event::{self}, KeyCode::{self}, KeyEvent, KeyModifiers
    },};

mod terminal;
mod view;

use terminal::{Terminal, Size, Position};
use view::View;
pub struct Editor {
    should_quit: bool,
    current_pos: Position,
    terminal_size: Size,
    view: View,
}

impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false, 
            current_pos : Position{x:0, y:0},
            terminal_size: Terminal::size().unwrap_or(Size { width: 80, height: 24 }),
            view: View::default(),
        }
    }

    pub fn new() -> Result<Self, Error>{
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1){
            view.load(file_name);
        } 
        Ok(Self::default())
    }

    pub fn run(&mut self){
        loop{
            self.refresh_screen();
            if self.should_quit {
                break;
            } 
            match read(){
                Ok(event) => self.evaluate_event(&event),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {e:?}");
                    }
                }
            }
        }
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
                        if self.current_pos.x != 0 {
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


    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor_to(self.current_pos);
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }

}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye ! \r\n");
        }
    }
}