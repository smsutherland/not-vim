//! Separates the mechanics of drawing an [`Editor`] from the internals of the editing itself.

use std::ops::{Deref, DerefMut};

use crate::tui::{rect::Bottom, Color, Frame, Rect, Style, Text};
use not_vim::editor::Editor;

/// An [`Editor`] which can be [`render`]ed.
///
/// This struct is a wrapper around [`Editor`] and [`Deref`]s to [`Editor`].
/// It stores extra information pertaining to how the contained [`Editor`] will be rendered.
///
/// [`render`]: EditorView::render
pub struct EditorView {
    /// The [`Editor`] being rendered.
    pub editor: Editor,
    /// The bottom status bar of the editor.
    status_bar: StatusBar,
    /// The position of the top-right corner of the view rectangle in the editor.
    view_pos: (usize, usize),
}

impl EditorView {
    /// Creates a new [`EditorView`].
    pub fn new(editor: Editor) -> Self {
        Self {
            editor,
            status_bar: StatusBar::default(),
            view_pos: (0, 0),
        }
    }

    /// Returns the position of the cursor in the editor.
    ///
    /// This is stored in `(row, column)` format.
    /// The editor stores this as `usize`s for indexing the text, but this function converts it to
    /// `u16`s to be used for rendering.
    pub fn selected_pos(&self) -> (u16, u16) {
        let (row, col) = self.editor.selected_pos();
        (row as u16, col as u16)
    }

    /// See [`frame`].
    ///
    /// [`frame`]: crate::tui::frame
    pub fn render(&self, frame: &mut Frame, region: Rect) {
        let regions = region.partition(Bottom);
        let bottom_bar = regions[0];
        let editor_area = regions[1];
        self.status_bar.render(frame, bottom_bar, {
            let pos = self.editor.selected_pos();
            (pos.0 as u16, pos.1 as u16)
        });

        let mut text = Text::from({
            let text = self.editor.text();
            let idx = text.line_to_char(self.view_pos.1);
            text.slice(idx..)
        });
        text.wrap(not_vim::config::WRAP_MODE);
        text.render(frame, editor_area);
    }

    pub fn resize(&mut self, new_size: (u16, u16)) {
        let editor_pos = self.editor.selected_pos();
        if editor_pos.1 < self.view_pos.1 {
            self.view_pos.1 = editor_pos.1;
        }
        // +1 because of line at the bottom for status bar.
        if editor_pos.1 - self.view_pos.1 + 1 >= new_size.1 as usize {
            self.view_pos.1 = editor_pos.1 + 2 - new_size.1 as usize;
        }
    }

    /// Get the current view position of the editor view
    pub fn view_pos(&self) -> (usize, usize) {
        self.view_pos
    }
}

impl Deref for EditorView {
    type Target = Editor;
    fn deref(&self) -> &Self::Target {
        &self.editor
    }
}

impl DerefMut for EditorView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.editor
    }
}

/// Placeholder struct for the bottom status bar of the editor.
///
/// Does not contain any information about the contents of the status_bar, but rather contains the
/// config for how the status bar will be rendered.
#[derive(Debug, Default)]
struct StatusBar {}

impl StatusBar {
    /// See [`frame`].
    ///
    /// [`frame`]: crate::tui::frame
    fn render(&self, frame: &mut Frame, region: Rect, position: (u16, u16)) {
        let bottom = region.top + region.height - 1;
        frame.set_style(Style::default().fg(Color::Black).bg(Color::White), region);
        let position = format!("{}:{}", position.1 + 1, position.0 + 1);
        for (x, c) in position.chars().enumerate() {
            frame.set_char(c, region.width - 15 + x as u16, bottom)
        }
    }
}
