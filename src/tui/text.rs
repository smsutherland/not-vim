//! [`Text`] can be drawn to the terminal here.
//!
//! TODO: more robust handling of multiline strings.
//! TODO: stylized strings.

use super::{Frame, Rect, Render};

/// A piece of text which can be drawn to the terminal.
pub struct Text<'a> {
    /// The content of the [`Text`].
    lines: &'a [String],
}

impl Render for Text<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        for (y, line) in self.lines.iter().take(region.height as usize).enumerate() {
            for (x, c) in line.chars().take(region.width as usize).enumerate() {
                let (x, y) = (x as u16, y as u16);
                frame.set_char(c, x + region.left, y + region.top);
            }
        }
    }
}

impl<'a> From<&'a [String]> for Text<'a> {
    fn from(value: &'a [String]) -> Self {
        Self { lines: value }
    }
}

/// A *single-line* piece of text which can be drawn to the terminal.
pub struct SingleText<'a> {
    /// The single line of text.
    ///
    /// Guaranteed to have no newlines in it.
    text: &'a str,
}

impl<'a> From<&'a String> for SingleText<'a> {
    fn from(value: &'a String) -> Self {
        Self {
            text: match value.find('\n') {
                Some(index) => &value[..index],
                None => value.as_str(),
            },
        }
    }
}

impl Render for SingleText<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        for (x, c) in self.text.chars().enumerate() {
            frame.set_char(c, x as u16 + region.left, region.top);
        }
    }
}
