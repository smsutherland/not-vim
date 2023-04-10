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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// The coordinate of the top side of the rectangle.
    pub top: u16,
    /// The coordinate of the left side of the rectangle.
    pub left: u16,
    /// Height of the rectangle.
    pub height: u16,
    /// Width of the rectangle
    pub width: u16,
}

impl Rect {
    /// Get a rect representing the current size of the terminal being written to.
    fn get_size() -> Self {
        let (width, height) = terminal::size().unwrap();
        Self {
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
    fn flush(&mut self) -> io::Result<()> {
        let diff = self.current_buf().diff(self.display_buf());

        for (cell, x, y) in diff {
            // potential optimization: don't queue a MoveTo if the previous character was right
            // before this one.
            queue!(self.stdout, MoveTo(x, y), Print(cell.symbol))?;
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

    // Concise description stolen from tui.
    /// Synchronizes terminal size, calls the rendering closure, flushes the current internal state and prepares for the next draw call.
    pub fn draw(&mut self, draw: impl Fn(&mut Frame) -> io::Result<()>) -> io::Result<()> {
        draw(&mut Frame {
            region: self.current_buf().area,
            buffer: self.current_buf_mut(),
        })?;
        self.flush()
    }
}

/// An abstraction around drawing to a region of a [Buffer].
pub struct Frame<'a> {
    /// The underlying [Buffer] being drawn to.
    buffer: &'a mut Buffer,
    /// The region of the [Buffer] being drawn to.
    region: Rect,
}

impl<'a> Frame<'a> {
    /// Draw a [Render]able item to the [Frame].
    pub fn render(&mut self, item: &impl Render) -> io::Result<()> {
        item.render(self)
    }

    /// Sets the char at a single location in the frame.
    pub fn set_char(&mut self, c: char, mut x: u16, mut y: u16) {
        // Should these panic or should the function return a Result?
        if x >= self.region.width {
            todo!("panic message");
        }
        if y >= self.region.height {
            todo!("panic message");
        }

        x += self.region.left;
        y += self.region.top;

        let i = x as usize + self.buffer.area.width as usize * y as usize;
        self.buffer.content[i].symbol = c;
    }

    /// Returns the region of this [`Frame`].
    pub fn region(&self) -> Rect {
        self.region
    }

    /// Clear the whole underlying buffer.
    pub fn clear(&mut self) {
        for y in self.region.top..(self.region.top + self.region.height) {
            for x in self.region.left..(self.region.left + self.region.width) {
                let i = self.buffer.area.width as usize * y as usize + x as usize;
                self.buffer.content[i] = Cell::default();
            }
        }
        self.buffer.content.fill(Cell::default())
    }
}

/// Take a [Frame] and draw to its underlying [Buffer].
///
/// This is currently achieved by mainly using the [Frame::set_char] method, which allows you to,
/// one character at a time, draw out the content being displayed. [Frame::render] can be called to
/// draw other objects implimenting [Render].
///
/// Example implimentation of [Render] on [String]:
/// ```
/// impl Render for String {
///     fn render(&self, frame: &mut Frame) -> io::Result<()> {
///         for (i, c) in self.chars().enumerate() {
///             frame.set_char(c, i, 0);
///         }
///         Ok(())
///     }
/// }
/// ```
pub trait Render {
    /// Take a [Frame] and draw to its underlying [Buffer].
    fn render(&self, frame: &mut Frame) -> io::Result<()>;
}
