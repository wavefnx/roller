use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::{
    error::Error,
    io::Stdout,
    ops::{Deref, DerefMut},
};

/// The `Terminal` struct represents a terminal instance.
///
/// It wraps the `ratatui::Terminal` with a `CrosstermBackend` and provides
/// methods for entering and exiting the terminal mode.
pub struct Terminal {
    /// The inner `ratatui::Terminal` instance with a `CrosstermBackend`.
    /// It implements the `Deref` and `DerefMut` which allow direct access to it.
    pub inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    /// Enters the terminal mode and returns a new `Terminal` instance.
    ///
    /// By enabling the terminals behavior changes,
    /// you can find out more by following this [link](https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode)
    ///
    /// We then instantiate a new `CrosstermBackend` with Stdout as a writer, `EnterAlternateScreen`, clean the screen and then return the Terminal instance.
    ///
    /// ### Returns
    /// A new instance of the `Terminal` struct, or an error if any part of the process fails.
    pub fn enter() -> Result<Self, Box<dyn Error>> {
        // [link](https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode)
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(std::io::stdout());
        let mut terminal = ratatui::Terminal::new(backend)?;
        execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            DisableMouseCapture
        )?;

        terminal.clear()?;

        Ok(Self { inner: terminal })
    }

    /// Exits the terminal mode and restores the previous terminal state.
    ///
    /// Reverts the changes made by `Self::enter`, clears the terminal and shows/resets the cursor to the top left corner.
    ///
    /// ### Returns
    /// An `Ok` result if the terminal was successfully exited or an error if any part of the process fails.
    pub fn exit(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(self.backend_mut(), LeaveAlternateScreen,)?;
        self.clear()?;
        self.set_cursor(0, 0)?;
        self.show_cursor()?;

        Ok(())
    }
}

/// Implements the `Deref` trait for `Terminal`.
///
/// This allows dereferencing a `Terminal` instance to access the underlying
/// `ratatui::Terminal<CrosstermBackend<Stdout>>` methods and properties directly.
impl Deref for Terminal {
    type Target = ratatui::Terminal<CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Implements the `DerefMut` trait for `Terminal`.
///
/// Mutably dereferencing a `Terminal` instance to access and modify the
/// underlying `ratatui::Terminal<CrosstermBackend<Stdout>>` methods and properties directly.
impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
