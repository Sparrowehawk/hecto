use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{queue, Command};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{stdout, Error, Write};


#[derive(Default, Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Represents the Terminal.Add commentMore actions
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the cursor out of these bounds, it will also be truncated.
pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alt_screen()?;
        Self::show_cursor()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alt_screen()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
        
    }

    fn leave_alt_screen() -> Result<(), Error>{
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    fn enter_alt_screen() -> Result<(), Error>{
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_cursor_to(Position { x: 0, y: row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }


    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), Error>{
        queue!(stdout(), command)?;
        Ok(())
    }
}
