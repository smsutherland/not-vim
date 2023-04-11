//! A [Frame] represents a single region of a terminal which can be drawn to.

use super::{Buffer, Cell, Rect, Render};
use std::io;

/// An abstraction around drawing to a region of a [Buffer].
pub struct Frame<'a> {
    /// The underlying [Buffer] being drawn to.
    pub(super) buffer: &'a mut Buffer,
    /// The region of the [Buffer] being drawn to.
    pub(super) region: Rect,
}

impl Frame<'_> {
    /// Draw a [Render]able item to the [Frame].
    #[inline]
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
