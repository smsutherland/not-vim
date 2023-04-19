//! [`Text`] can be drawn to the terminal here.
//!
//! TODO: more robust handling of multiline strings.
//! TODO: stylized strings.

use super::{Frame, Rect, Render};
use bitflags::bitflags;
use crossterm::{
    style::{Color, SetBackgroundColor, SetForegroundColor},
    Command,
};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    fg: Color,
    bg: Color,
    modifiers: Modifier,
}

impl Style {
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn diff(&self, other: Self) -> StyleChange {
        StyleChange {
            fg: if self.fg != other.fg {
                Some(other.fg)
            } else {
                None
            },
            bg: if self.bg != other.bg {
                Some(other.bg)
            } else {
                None
            },
            add_modifier: other.modifiers,
            sub_modifier: self.modifiers,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::White,
            bg: Color::Black,
            modifiers: Modifier::empty(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StyleChange {
    fg: Option<Color>,
    bg: Option<Color>,
    add_modifier: Modifier,
    sub_modifier: Modifier,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const UNDERLINED        = 0b0000_0000_1000;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

impl Command for StyleChange {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        if let Some(fg) = self.fg {
            SetForegroundColor(fg).write_ansi(f)?;
        }
        if let Some(bg) = self.bg {
            SetBackgroundColor(bg).write_ansi(f)?;
        }
        // TODO: handle modifiers too
        Ok(())
    }
}
