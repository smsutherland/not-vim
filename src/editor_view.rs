//! Separates the mechanics of drawing an [`Editor`] from the internals of the editing itself.

use std::ops::{Deref, DerefMut};

use crate::{
    editor::Editor,
    tui::{rect::Bottom, Color, Frame, Rect, Render, Style, Text},
};

/// An [`Editor`] which can be [`Render`]ed.
///
/// This struct is a wrapper around [`Editor`] and [`Deref`]s to [`Editor`].
/// It stores extra information pertaining to how the contained [`Editor`] will be rendered.
pub struct EditorView {
    pub editor: Editor,
    /// The bottom status bar of the editor.
    status_bar: StatusBar,
}

impl EditorView {
    /// Creates a new [`EditorView`].
    pub fn new(editor: Editor) -> Self {
        Self {
            editor,
            status_bar: StatusBar::default(),
        }
    }

    pub fn selected_pos(&self) -> (u16, u16) {
        let (row, col) = self.editor.selected_pos();
        (row as u16, col as u16)
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

impl Render for EditorView {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let regions = region.partition(Bottom);
        let bottom_bar = regions[0];
        let editor_area = regions[1];
        let bar = self.status_bar.make_renderable({
            let pos = self.editor.selected_pos();
            (pos.0 as u16, pos.1 as u16)
        });
        frame.render(&bar, bottom_bar);

        let mut text = Text::from(self.editor.text());
        text.wrap(crate::config::WRAP_MODE);
        frame.render(&text, editor_area);
    }
}

/// Placeholder struct for the bottom status bar of the editor.
///
/// Does not contain any information about the contents of the status_bar, but rather contains the
/// config for how the status bar will be rendered.
#[derive(Debug, Default)]
struct StatusBar {}

impl StatusBar {
    fn make_renderable(&self, position: (u16, u16)) -> RenderableStatusBar {
        RenderableStatusBar {
            _config: self,
            position,
        }
    }
}

/// The bottom status bar filled with all the information it requires to properly render
struct RenderableStatusBar<'a> {
    /// [`StatusBar`] configuration.
    _config: &'a StatusBar,
    /// The position of the cursor in the editor.
    position: (u16, u16),
}

impl Render for RenderableStatusBar<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let bottom = region.top + region.height - 1;
        frame.set_style(Style::default().fg(Color::Black).bg(Color::White), region);
        let position = format!("{}:{}", self.position.1 + 1, self.position.0 + 1);
        for (x, c) in position.chars().enumerate() {
            frame.set_char(c, region.width - 15 + x as u16, bottom)
        }
    }
}
