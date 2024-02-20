//! Definitions and constant values for matching against.
//!
//! This includes things like keybinds for specific actions.

pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor_view::Mode;

/// Read an event and translate it into a [`Message`].
///
/// This provides an easily-configurable layer in which to transform from user events to actions
/// for the editor.
pub fn translate_event(mode: Mode, key: Key) -> Message {
    match mode {
        Mode::Normal => normal_mode_event(key),
        Mode::Insert => insert_mode_event(key),
    }
}

/// Translate a [`KeyEvent`] into a [`Message`] for normal mode.
fn normal_mode_event(key: Key) -> Message {
    match key {
        Key {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        } => Message::Quit,

        Key {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
        } => Message::Write,

        Key {
            code: KeyCode::Left | KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
        } => Message::Left,

        Key {
            code: KeyCode::Right | KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
        } => Message::Right,

        Key {
            code: KeyCode::Up | KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
        } => Message::Up,

        Key {
            code: KeyCode::Down | KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
        } => Message::Down,

        Key {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
        } => Message::Mode(Mode::Insert),

        _ => Message::None,
    }
}

/// Translate a [`KeyEvent`] into a [`Message`] for insert mode.
fn insert_mode_event(key: Key) -> Message {
    match key {
        Key {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => Message::Enter,

        Key {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
        } => Message::Backspace,

        Key {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        } => Message::Left,

        Key {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        } => Message::Right,

        Key {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        } => Message::Up,

        Key {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        } => Message::Down,

        Key {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        } => Message::Mode(Mode::Normal),

        Key {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
        } => Message::Char(c),

        _ => Message::None,
    }
}

/// An enumeration of all possible actions the editor could take.
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Quit the editor.
    Quit,
    /// Write the current buffer to its file.
    Write,
    /// Enter a newline.
    Enter,
    /// Delete the character behind the cursor.
    Backspace,
    /// Move the cursor left.
    Left,
    /// Move the cursor right.
    Right,
    /// Move the cursor up.
    Up,
    /// Move the cursor down.
    Down,
    /// Insert a character.
    Char(char),
    /// Enter a given [`Mode`].
    Mode(Mode),
    /// Do nothing.
    None,
}

/// The configured wrap mode for the editor.
pub const WRAP_MODE: WrapMode = WrapMode::NoWrap(Some('>'));

/// A keybind for a specific action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    /// Which key was pressed.
    pub code: KeyCode,
    /// Any modifiers on the pressed key.
    pub modifiers: KeyModifiers,
}

impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        Self {
            code: value.code,
            modifiers: value.modifiers,
        }
    }
}

/// Enumeration of possible ways of handling lines which are longer than editor width.
#[allow(dead_code)] // Only one variant will be used in the configuration.
pub enum WrapMode {
    /// Long lines will continue to the edge of the screen. Any excess gets displayed on the
    /// next line. Note that this is only a display effect. No newlines are inserted when wrapping
    /// text.
    Wrap,
    /// Long lines will cut off at the edge of the screen and the provided char will be placed at
    /// the end to siginfy that the line continues off the screen. If the provided character is
    /// [`None`] then nothing will be displayed to signify line continuance.
    NoWrap(Option<char>),
}
