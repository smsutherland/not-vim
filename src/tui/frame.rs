//! A [`Frame`] represents a single region of a terminal which can be drawn to.
//!
//! [`Frame`]s are usually rendered to via a `render` method on a struct.
//! This is not implemented as a trait to allow each struct to define its own mutability and other
//! parameters for rendering.
//!
//! This is currently achieved by mainly using the [`Frame::set_char`] method, which allows you to,
//! one character at a time, draw out the content being displayed.
//! Example implimentation of `render` on [`String`]:
//! ```
//! impl String {
//!     fn render(&self, frame: &mut Frame, region: Rect) {
//!         for (i, c) in self.chars().enumerate() {
//!             frame.set_char(c, region.left + i, region.top);
//!         }
//!     }
//! }
//! ```
//!

use super::{Buffer, Rect, Style};

/// An abstraction around drawing to a region of a [`Buffer`].
pub struct Frame<'a> {
    /// The underlying [`Buffer`] being drawn to.
    pub(super) buffer: &'a mut Buffer,
}

impl Frame<'_> {
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

    /// Set the [`Style`] of all the [`Cell`]s in the underlying [`Buffer`] in the region specified.
    ///
    /// [`Cell`]: super::Cell
    pub fn set_style(&mut self, style: Style, region: Rect) {
        for y in region.top..region.top + region.height {
            for x in region.left..region.left + region.width {
                let i = x as usize + self.buffer.area.width as usize * y as usize;
                self.buffer.content[i].style = style;
            }
        }
    }
}
