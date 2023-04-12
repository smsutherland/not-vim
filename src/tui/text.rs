//! [`Text`] can be drawn to the terminal here.
//!
//! TODO: more robust handling of multiline strings.
//! TODO: stylized strings.

use super::{Frame, Rect, Render};

/// A piece of text which can be drawn to the terminal.
pub struct Text {
    /// The content of the [`Text`].
    text: String,
}

impl Render for Text {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let mut x = 0;
        let mut y = 0;
        for c in self.text.chars() {
            if c == '\n' {
                y += 1;
                x = 0;
            } else {
                frame.set_char(c, x + region.left, y + region.top);
                x += 1;
            }
        }
    }
}

impl From<&[String]> for Text {
    fn from(value: &[String]) -> Self {
        Self {
            text: value.join("\n"),
        }
    }
}
