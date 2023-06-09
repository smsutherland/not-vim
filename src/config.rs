//! Definitions and constant values for matching against.
//!
//! This includes things like keybinds for specific actions.

pub use configurable::*;
pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
pub use non_configurable::*;

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

mod configurable {
    //! These are the keybinds which are worth configuring.

    use super::*;

    /// Quit the editor.
    pub const QUIT: Key = Key {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::CONTROL,
    };

    /// Write the current buffer to its file.
    pub const WRITE: Key = Key {
        code: KeyCode::Char('w'),
        modifiers: KeyModifiers::CONTROL,
    };
}

mod non_configurable {
    //! These are keybinds which really shouldn't be touched.

    use super::*;

    /// Enter a newline.
    pub const ENTER: Key = Key {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
    };

    /// Delete the character behind the cursor.
    pub const BACKSPACE: Key = Key {
        code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
    };

    /// Move the cursor left.
    pub const LEFT: Key = Key {
        code: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
    };

    /// Move the cursor right.
    pub const RIGHT: Key = Key {
        code: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
    };

    /// Move the cursor up.
    pub const UP: Key = Key {
        code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
    };

    /// Move the cursor down.
    pub const DOWN: Key = Key {
        code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
    };
}
