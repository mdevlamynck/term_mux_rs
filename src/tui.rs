//! Terminal UI library
use termion;

/// A rectangular size in number of columns and rows
pub struct Size {
    /// Number of columns
    pub width:  u16,
    /// Number of rows
    pub height: u16,
}

/// Returns the terminal current size
pub fn get_terminal_size() -> Result<Size, ()> {
    let (width, height) = termion::terminal_size().map_err(|_| ())?;
    Ok( Size { width, height } )
}
