//! Here's all the code for abstracting a terminal.
//!
//! Contains information about [`Buffer`]s and individual [`Cell`]s.

pub mod frame;
pub mod rect;
mod text;

pub use crossterm::style::Color;
use crossterm::{cursor::MoveTo, queue, style::Print};
pub use frame::Frame;
pub use rect::Rect;
use std::io::{self, StdoutLock, Write};
pub use text::{Style, Text};

/// All the information regarding the content of a single cell of a terminal.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    /// Which character is at this location.
    symbol: char,
    /// [`Style`] of the character.
    style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            style: Style::default(),
        }
    }
}

/// A buffer of [`Cell`]s.
///
/// Represents the content of a region of the terminal.
#[derive(Debug, Clone)]
struct Buffer {
    /// All the [`Cell`]s of the buffer, stored in row-major order.
    content: Vec<Cell>,
    /// The area the [`Buffer`] is representing.
    area: Rect,
}

impl Buffer {
    /// Takes another [`Buffer`] and returns a vector of all the [`Cell`]s which are different between
    /// `self` and the other [`Buffer`].
    ///
    /// This vector also contains the positions of the cells.
    fn diff(&self, other: &Self) -> Vec<(Cell, u16, u16)> {
        if self.area != other.area {
            enumerate_2d(&self.content, self.area).collect()
        } else {
            enumerate_2d(&self.content, self.area)
                .filter(|(cell, x, y)| {
                    let other_cell =
                        other.content[*y as usize * self.area.width as usize + *x as usize];
                    *cell != other_cell
                })
                .collect()
        }
    }

    /// Resizes a buffer to match the area of `new_area`.
    ///
    /// This will truncate any [`Cell`]s which fall outside of the region and will insert blank cells
    /// if the new area is larger than the previous area.
    fn resize(&mut self, new_area: Rect) {
        self.area = new_area;
        self.content.resize(
            new_area.width as usize * new_area.height as usize,
            Cell::default(),
        );
    }

    /// Fill the entire buffer with blank spaces.
    fn clear(&mut self) {
        self.content.fill_with(Cell::default);
    }
}

/// Take a vector of [`Cell`]s and enumerate them with their 2d coordinates.
///
/// The coordinates are found by mapping the vector in a row-major fashion to the area described by
/// `area`.
fn enumerate_2d(items: &[Cell], area: Rect) -> impl Iterator<Item = (Cell, u16, u16)> + '_ {
    assert_eq!(
        items.len(),
        area.width as usize * area.height as usize,
        "{area:?}"
    );
    items.iter().enumerate().map(move |(i, item)| {
        (
            *item,
            (i % area.width as usize) as u16,
            (i / area.width as usize) as u16,
        )
    })
}

impl Default for Buffer {
    fn default() -> Self {
        let area = Rect::get_size();

        let content = vec![Cell::default(); area.height as usize * area.width as usize];
        Self { content, area }
    }
}

/// Representation of a terminal which can be written to and displayed.
#[derive(Debug)]
pub struct Terminal {
    /// The write buffer and the display buffer.
    buffers: [Buffer; 2],
    /// Which buffer is being written to.
    ///
    /// The `current_buf` is being written to and
    /// The `1 - current_buf` is currently being displayed.
    current_buf: usize,
    /// The writer being used to write the editor to.
    stdout: StdoutLock<'static>,
}

impl Terminal {
    /// Create a Terminal around standard out.
    pub fn new() -> Self {
        Self {
            buffers: [Buffer::default(), Buffer::default()],
            current_buf: 0,
            stdout: io::stdout().lock(),
        }
    }

    /// Write the contents of the current [`Buffer`] to the terminal.
    ///
    /// This will draw the current [`Buffer`], then swap the current and back buffers.
    /// The new current buffer is made into a copy of the new back buffer (the one which just got
    /// drawn to the terminal).
    fn flush(&mut self, final_position: Option<(u16, u16)>) -> anyhow::Result<()> {
        let diff = self.current_buf().diff(self.display_buf());

        let mut prev_style = Style::default();
        let mut prev_position = None;

        for (cell, x, y) in diff {
            if prev_position
                .map(|(old_x, old_y)| (x, y) != (old_x + 1, old_y))
                .unwrap_or(true)
            {
                queue!(self.stdout, MoveTo(x, y))?;
            }
            prev_position = Some((x, y));
            let style_diff = cell.style.diff(prev_style);
            prev_style = cell.style;
            queue!(self.stdout, style_diff, Print(cell.symbol))?;
        }

        if let Some((x, y)) = final_position {
            queue!(self.stdout, MoveTo(x, y))?;
        }
        // reset the style
        queue!(self.stdout, Style::default().diff(prev_style))?;

        self.stdout.flush()?;

        // swap buffers
        self.current_buf = 1 - self.current_buf;
        *self.current_buf_mut() = self.buffers[1 - self.current_buf].clone();

        Ok(())
    }

    // /// Set the symbol at index `i` to `c`.
    // pub fn set(&mut self, c: char, i: usize) {
    //     self.current_buf_mut().content[i] = Cell { symbol: c }
    // }
    //
    // /// Move the cursor to the position represented by the index `i`.
    // pub fn set_cursor(&mut self, i: usize) -> anyhow::Result<()> {
    //     execute!(
    //         self.stdout,
    //         MoveTo(
    //             (i % self.buffers[self.current_buf].area.width as usize) as u16,
    //             (i / self.buffers[self.current_buf].area.width as usize) as u16,
    //         )
    //     )?;
    //     Ok(())
    // }

    /// Resize the [`Terminal`] to reflect the actual size of the terminal.
    pub fn resize(&mut self) {
        let area = Rect::get_size();
        self.current_buf_mut().resize(area);
    }

    /// Get a reference to the [`Buffer`] currently being written to.
    fn current_buf(&self) -> &Buffer {
        &self.buffers[self.current_buf]
    }

    /// Get a reference to the [`Buffer`] currently being displayed in the terminal.
    fn display_buf(&self) -> &Buffer {
        &self.buffers[1 - self.current_buf]
    }

    /// Get a mutable reference to the [`Buffer`] currently being written to.
    fn current_buf_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buf]
    }

    // /// Get a mutable reference to the [`Buffer`] currently being displayed in the terminal.
    // /// This function does not exist because the display [`Buffer`] shouldn't be modified.
    // fn display_buf_mut(&mut self) -> &mut Buffer {
    //     &mut self.buffers[1 - self.current_buf]
    // }

    // Concise description stolen from tui.
    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state and prepares for the next draw call.
    ///
    /// If the return value is [`Some`], then this will set the position of the cursor to the
    /// returned coordinates.
    pub fn draw(&mut self, draw: impl Fn(&mut Frame) -> Option<(u16, u16)>) -> anyhow::Result<()> {
        self.current_buf_mut().clear();
        let final_position = draw(&mut Frame {
            buffer: self.current_buf_mut(),
        });
        self.flush(final_position)
    }
}
