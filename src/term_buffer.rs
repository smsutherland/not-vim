//! Here's all the code for abstracting a terminal.
//!
//! Contains information about [Buffer]s and individual [Cell]s.

use std::io::{self, Stdout, Write};

use crossterm::{cursor::MoveTo, execute, queue, style::Print, terminal};

/// All the information regarding the content of a single cell of a terminal.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    /// Which character is at this location.
    symbol: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { symbol: ' ' }
    }
}

/// A buffer of [Cell]s.
///
/// Represents the content of a region of the terminal.
#[derive(Debug, Clone)]
struct Buffer {
    /// All the [Cell]s of the buffer, stored in row-major order.
    content: Vec<Cell>,
    /// The area the [Buffer] is representing.
    area: Rect,
}

impl Buffer {
    /// Takes another [Buffer] and returns a vector of all the [Cell]s which are different between
    /// `self` and the other [Buffer].
    ///
    /// This vector also contains the positions of the cells.
    fn diff(&self, other: &Buffer) -> Vec<(Cell, u16, u16)> {
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
    /// This will truncate any [Cell]s which fall outside of the region and will insert blank cells
    /// if the new area is larger than the previous area.
    fn resize(&mut self, new_area: Rect) {
        self.area = new_area;
        self.content.resize(
            new_area.width as usize * new_area.height as usize,
            Cell::default(),
        );
    }
}

/// Take a vector of Cells and enumerate them with their 2d coordinates.
///
/// The coordinates are found by mapping the vector in a row-major fashion to the area described by
/// `area`.
fn enumerate_2d(items: &Vec<Cell>, area: Rect) -> impl Iterator<Item = (Cell, u16, u16)> + '_ {
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

/// A simple struct representing a rectangular region of the terminal.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Rect {
    /// The coordinate of the top side of the rectangle.
    top: u16,
    /// The coordinate of the left side of the rectangle.
    left: u16,
    /// Height of the rectangle.
    height: u16,
    /// Width of the rectangle
    width: u16,
}

impl Rect {
    /// Get a rect representing the current size of the terminal being written to.
    fn get_size() -> Self {
        let (width, height) = terminal::size().unwrap();
        Rect {
            top: 0,
            left: 0,
            height,
            width,
        }
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
    stdout: Stdout,
}

impl Terminal {
    /// Create a Terminal around Stdout.
    pub fn new() -> Self {
        Self {
            buffers: [Buffer::default(), Buffer::default()],
            current_buf: 0,
            stdout: io::stdout(),
        }
    }

    /// Write the contents of the current [Buffer] to the terminal.
    ///
    /// This will draw the current [Buffer], then swap the current and back buffers.
    /// The new current buffer is made into a copy of the new back buffer (the one which just got
    /// drawn to the terminal).
    pub fn draw(&mut self) -> io::Result<()> {
        let diff = self.current_buf().diff(self.display_buf());

        for (cell, x, y) in diff {
            queue!(self.stdout, MoveTo(x, y))?;
            queue!(self.stdout, Print(cell.symbol))?;
        }

        self.stdout.flush()?;

        // swap buffers
        self.current_buf = 1 - self.current_buf;
        *self.current_buf_mut() = self.buffers[1 - self.current_buf].clone();

        Ok(())
    }

    /// Set the symbol at index `i` to `c`.
    pub fn set(&mut self, c: char, i: usize) {
        self.current_buf_mut().content[i] = Cell { symbol: c }
    }

    /// Move the cursor to the position represented by the index `i`.
    pub fn set_cursor(&mut self, i: usize) -> io::Result<()> {
        execute!(
            self.stdout,
            MoveTo(
                (i % self.buffers[self.current_buf].area.width as usize) as u16,
                (i / self.buffers[self.current_buf].area.width as usize) as u16,
            )
        )?;
        Ok(())
    }

    /// Resize the Terminal to reflect the actual size of the terminal.
    pub fn resize(&mut self) {
        let area = Rect::get_size();
        self.current_buf_mut().resize(area);
    }

    /// Get a reference to the [Buffer] currently being written to.
    fn current_buf(&self) -> &Buffer {
        &self.buffers[self.current_buf]
    }

    /// Get a reference to the [Buffer] currently being displayed in the terminal.
    fn display_buf(&self) -> &Buffer {
        &self.buffers[1 - self.current_buf]
    }

    /// Get a mutable reference to the [Buffer] currently being written to.
    fn current_buf_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buf]
    }

    // /// Get a mutable reference to the [Buffer] currently being displayed in the terminal.
    // /// This function does not exist because the display [Buffer] shouldn't be modified.
    // fn display_buf_mut(&mut self) -> &mut Buffer {
    //     &mut self.buffers[1 - self.current_buf]
    // }
}
