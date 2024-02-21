//! [`Text`] can be drawn to the terminal here.
//!
//! TODO: more robust handling of multiline strings.
//! TODO: stylized strings.

use crate::{config::WrapMode, editor::trim_newlines};

use super::{Frame, Rect, Render};
use bitflags::bitflags;
use crossterm::{
    style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor},
    Command,
};
use ropey::RopeSlice;

/// A piece of text which can be drawn to the terminal.
pub struct Text<'a> {
    /// The content of the [`Text`].
    text: RopeSlice<'a>,
    /// How this box of text will wrap it's contents.
    ///
    /// See [`WrapMode`]. Defaults to [`WrapMode::NoWrap(None)`]
    ///
    /// [`WrapMode::NoWrap(None)`]: WrapMode::NoWrap
    wrap_mode: WrapMode,
}

impl<'a> Text<'a> {
    /// Change how this box of text wraps.
    ///
    /// See [`WrapMode`].
    pub fn wrap(&mut self, wrap_mode: WrapMode) {
        self.wrap_mode = wrap_mode;
    }

    /// Renders the text in the case where `self.wrap_mode` is set to [`WrapMode::Wrap`].
    fn render_no_wrap(&self, frame: &mut Frame, region: Rect) {
        for (y, line) in self
            .text
            .lines()
            .take(region.height as usize)
            .map(trim_newlines)
            .enumerate()
        {
            for (x, c) in line.chars().take(region.width as usize).enumerate() {
                let (x, y) = (x as u16, y as u16);
                frame.set_char(c, x + region.left, y + region.top);
            }
        }
    }

    /// Renders the text in the case where `self.wrap_mode` is set to [`WrapMode::NoWrap(Some(c))`], where c is passed in as a parameter here.
    ///
    /// [`WrapMode::NoWrap(Some(c))`]: WrapMode::NoWrap
    fn render_no_wrap_with_char(&self, frame: &mut Frame, region: Rect, c: char) {
        for (y, line) in self
            .text
            .lines()
            .take(region.height as usize)
            .map(trim_newlines)
            .enumerate()
        {
            for (x, c) in line.chars().take(region.width as usize).enumerate() {
                let (x, y) = (x as u16, y as u16);
                frame.set_char(c, x + region.left, y + region.top);
            }
            if line.len_chars() > region.width as usize {
                frame.set_char(c, region.width - 1 + region.left, y as u16 + region.top);
            }
        }
    }

    /// Renders the text in the case where `self.wrap_mode` is set to [`WrapMode::NoWrap(None)`].
    ///
    /// [`WrapMode::NoWrap(None)`]: WrapMode::NoWrap
    fn render_wrap(&self, frame: &mut Frame, region: Rect) {
        let mut y = 0;

        for line in self
            .text
            .lines()
            .take(region.height as usize)
            .map(trim_newlines)
        {
            let mut x = 0;
            for c in line.chars() {
                frame.set_char(c, x + region.left, y + region.top);

                x += 1;
                if x == region.width {
                    x = 0;
                    y += 1;
                }
            }

            y += 1;
            if y == region.height {
                break;
            }
        }
    }
}

impl Render for Text<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        match self.wrap_mode {
            WrapMode::Wrap => self.render_wrap(frame, region),
            WrapMode::NoWrap(Some(c)) => self.render_no_wrap_with_char(frame, region, c),
            WrapMode::NoWrap(None) => self.render_no_wrap(frame, region),
        }
    }
}

impl<'a, T> From<T> for Text<'a>
where
    T: Into<RopeSlice<'a>>,
{
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            wrap_mode: WrapMode::NoWrap(None),
        }
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

/// Represents the style a [`Cell`] can have.
/// Includes a foreground and background [`Color`]s as well as any [`Modifier`]s applied.
///
/// [`Style`]s have a builder-like pattern for construction. For example, to create a [`Style`] with a foreground color of red which is underlined and bolded:
/// ```
/// let style = Style::default()
///     .fg(Color::Red)
///     .add_modifier(Modifier::UNDERLINED)
///     .add_modifier(Modifier::BOLD);
/// ```
///
/// Because [`Modifier`]s are [`bitflags`], This can be compacted slightly to be:
/// ```
/// let style = Style::default()
///     .fg(Color::Red)
///     .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
/// ```
///
/// When using a [`Frame`] to render, use the [`set_style`] method to set the style of a region of the [`Buffer`]
///
/// [`Cell`]: super::Cell
/// [`bitflags`]: ::bitflags
/// [`set_style`]: Frame::set_style
/// [`Buffer`]: super::Buffer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The foreground [`Color`].
    fg: Color,
    /// The background [`Color`].
    bg: Color,
    /// Which [`Modifier`]s are active for this [`Style`].
    modifiers: Modifier,
}

impl Style {
    /// Set the foreground color of the [`Style`].
    ///
    /// Returns Self to allow method chaining.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set the background color of the [`Style`].
    ///
    /// Returns Self to allow method chaining.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Take self and add a [`Modifier`] on to it.
    ///
    /// Returns Self to allow method chaining.
    #[allow(dead_code)]
    pub fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers |= modifier;
        self
    }

    /// Find the [`StyleChange`] required to move from `prev_style` to `self`.
    pub fn diff(&self, prev_style: Self) -> StyleChange {
        StyleChange {
            fg: if self.fg != prev_style.fg {
                Some(self.fg)
            } else {
                None
            },
            bg: if self.bg != prev_style.bg {
                Some(self.bg)
            } else {
                None
            },
            add_modifier: self.modifiers - prev_style.modifiers,
            sub_modifier: prev_style.modifiers - self.modifiers,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifier::empty(),
        }
    }
}

/// Represents a _change_ in the style of the terminal.
#[derive(Debug, Clone, Copy)]
pub struct StyleChange {
    /// If the foreground color needs to change, it is specified here as `Some(Color)`. If no
    /// foreground change is needed, this is `None`.
    fg: Option<Color>,
    /// If the background color needs to change, it is specified here as `Some(Color)`. If no
    /// background change is needed, this is `None`.
    bg: Option<Color>,
    /// Set of [`Modifier`]s which are being added in this style change.
    add_modifier: Modifier,
    /// Set of [`Modifier`]s which are being removed in this style change.
    sub_modifier: Modifier,
}

bitflags! {
    /// Set of all possible modifiers the terminal can put on a [`Cell`].
    ///
    /// TODO: determine which ones are not used because bitflags forces it to `allow(dead_code)`.
    ///
    /// [`Cell`]: super::Cell
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Modifier: u16 {
        const BOLD        = 0b0000_0000_0001;
        const DIM         = 0b0000_0000_0010;
        const ITALIC      = 0b0000_0000_0100;
        const UNDERLINED  = 0b0000_0000_1000;
        const SLOW_BLINK  = 0b0000_0001_0000;
        const RAPID_BLINK = 0b0000_0010_0000;
        const REVERSED    = 0b0000_0100_0000;
        const HIDDEN      = 0b0000_1000_0000;
        const CROSSED_OUT = 0b0001_0000_0000;
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

        if self.sub_modifier.contains(Modifier::REVERSED) {
            SetAttribute(Attribute::NoReverse).write_ansi(f)?;
        }
        if self.sub_modifier.contains(Modifier::BOLD) {
            SetAttribute(Attribute::NormalIntensity).write_ansi(f)?;
        }
        if self.sub_modifier.contains(Modifier::ITALIC) {
            SetAttribute(Attribute::NoItalic).write_ansi(f)?;
        }
        if self.sub_modifier.contains(Modifier::UNDERLINED) {
            SetAttribute(Attribute::NoUnderline).write_ansi(f)?;
        }
        if self.sub_modifier.contains(Modifier::DIM) {
            SetAttribute(Attribute::NormalIntensity).write_ansi(f)?;
        }
        if self.sub_modifier.contains(Modifier::CROSSED_OUT) {
            SetAttribute(Attribute::NotCrossedOut).write_ansi(f)?;
        }
        if self.sub_modifier.contains(Modifier::SLOW_BLINK)
            || self.sub_modifier.contains(Modifier::RAPID_BLINK)
        {
            SetAttribute(Attribute::NoBlink).write_ansi(f)?;
        }

        if self.add_modifier.contains(Modifier::REVERSED) {
            SetAttribute(Attribute::Reverse).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::BOLD) {
            SetAttribute(Attribute::Bold).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::ITALIC) {
            SetAttribute(Attribute::Italic).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::UNDERLINED) {
            SetAttribute(Attribute::Underlined).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::DIM) {
            SetAttribute(Attribute::Dim).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::CROSSED_OUT) {
            SetAttribute(Attribute::CrossedOut).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::SLOW_BLINK) {
            SetAttribute(Attribute::SlowBlink).write_ansi(f)?;
        }
        if self.add_modifier.contains(Modifier::RAPID_BLINK) {
            SetAttribute(Attribute::RapidBlink).write_ansi(f)?;
        }

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        if let Some(fg) = self.fg {
            SetForegroundColor(fg).execute_winapi()?;
        }
        if let Some(bg) = self.bg {
            SetBackgroundColor(bg).execute_winapi()?;
        }

        if self.sub_modifier.contains(Modifier::REVERSED) {
            SetAttribute(Attribute::NoReverse).execute_winapi()?;
        }
        if self.sub_modifier.contains(Modifier::BOLD) {
            SetAttribute(Attribute::NormalIntensity).execute_winapi()?;
        }
        if self.sub_modifier.contains(Modifier::ITALIC) {
            SetAttribute(Attribute::NoItalic).execute_winapi()?;
        }
        if self.sub_modifier.contains(Modifier::UNDERLINED) {
            SetAttribute(Attribute::NoUnderline).execute_winapi()?;
        }
        if self.sub_modifier.contains(Modifier::DIM) {
            SetAttribute(Attribute::NormalIntensity).execute_winapi()?;
        }
        if self.sub_modifier.contains(Modifier::CROSSED_OUT) {
            SetAttribute(Attribute::NotCrossedOut).execute_winapi()?;
        }
        if self.sub_modifier.contains(Modifier::SLOW_BLINK)
            || self.sub_modifier.contains(Modifier::RAPID_BLINK)
        {
            SetAttribute(Attribute::NoBlink).execute_winapi()?;
        }

        if self.add_modifier.contains(Modifier::REVERSED) {
            SetAttribute(Attribute::Reverse).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::BOLD) {
            SetAttribute(Attribute::Bold).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::ITALIC) {
            SetAttribute(Attribute::Italic).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::UNDERLINED) {
            SetAttribute(Attribute::Underlined).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::DIM) {
            SetAttribute(Attribute::Dim).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::CROSSED_OUT) {
            SetAttribute(Attribute::CrossedOut).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::SLOW_BLINK) {
            SetAttribute(Attribute::SlowBlink).execute_winapi()?;
        }
        if self.add_modifier.contains(Modifier::RAPID_BLINK) {
            SetAttribute(Attribute::RapidBlink).execute_winapi()?;
        }

        Ok(())
    }
}
