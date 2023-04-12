//! A [`Frame`] represents a single region of a terminal which can be drawn to.

use super::{Buffer, Rect, Render};

/// An abstraction around drawing to a region of a [`Buffer`].
pub struct Frame<'a> {
    /// The underlying [`Buffer`] being drawn to.
    pub(super) buffer: &'a mut Buffer,
}

impl Frame<'_> {
    /// Draw a [`Render`]able item to the [`Frame`].
    #[inline]
    pub fn render(&mut self, item: &impl Render, region: Rect) {
        item.render(self, region);
    }

    /// Sets the char at a single location in the frame.
    pub fn set_char(&mut self, c: char, x: u16, y: u16) {
        // Should these panic or should the function return a Result?
        if x >= self.buffer.area.width {
            return;
            // todo!("panic message");
        }
        if y >= self.buffer.area.height {
            return;
            // todo!("panic message");
        }

        let i = x as usize + self.buffer.area.width as usize * y as usize;
        self.buffer.content[i].symbol = c;
    }

    /// Get the [`Rect`] representing the size of the [`Buffer`] being written to.
    pub fn size(&self) -> Rect {
        self.buffer.area
    }

    /// Clear the whole underlying [`Buffer`] in the region specified.
    pub fn clear(&mut self) {
        self.buffer.content.fill_with(Default::default);
    }
}
