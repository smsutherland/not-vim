//! Separates the mechanics of drawing an [`Editor`] from the internals of the editing itself.
//!
//! [`Editor`]: EditorInternal

use std::ops::Deref;

use crate::{
    editor::Editor as EditorInternal,
    tui::{rect::Bottom, Frame, Rect, Render, Text},
};

/// An [`Editor`] which can be [`Render`]ed.
///
/// [`Editor`]: EditorInternal
pub struct Editor<'a> {
    /// The bottom bar showing the status of the [`Editor`].
    ///
    /// [`Editor`]: EditorInternal
    status_bar: StatusBar,
    /// The main section showing the editing region.
    edit_area: EditArea<'a>,
}

impl Editor<'_> {
    /// Returns the cursor pos of this [`Editor`].
    pub fn cursor_pos(&self) -> (u16, u16) {
        self.edit_area.cursor_pos()
    }
}

impl<'a> From<&'a EditorInternal> for Editor<'a> {
    fn from(value: &'a EditorInternal) -> Self {
        Self {
            status_bar: StatusBar {},
            edit_area: EditArea { editor: value },
        }
    }
}

impl Render for Editor<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let regions = region.partition(Bottom);
        let bottom_bar = regions[0];
        let editor_area = regions[1];
        frame.render(&self.status_bar, bottom_bar);
        frame.render(&self.edit_area, editor_area);
    }
}

/// Newtype around [`Editor`] to enable allow [`Render`]ing.
///
/// [`Editor`]: EditorInternal
struct EditArea<'a> {
    /// The [`Editor`] being drawn.
    ///
    /// [`Editor`]: EditorInternal
    editor: &'a EditorInternal,
}

impl Deref for EditArea<'_> {
    type Target = EditorInternal;

    fn deref(&self) -> &Self::Target {
        self.editor
    }
}

impl Render for EditArea<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let text = Text::from(self.editor.lines());
        frame.render(&text, region);
    }
}

/// Placeholder struct for the bottom status bar of the editor.
#[derive(Debug, Default)]
struct StatusBar {}

impl Render for StatusBar {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let bottom = region.top + region.height - 1;
        for x in 0..region.width {
            frame.set_char('█', x, bottom);
        }
    }
}
