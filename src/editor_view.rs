//! Separates the mechanics of drawing an [`Editor`] from the internals of the editing itself.
//!
//! [`Editor`]: EditorInternal

use std::ops::Deref;

use crate::{
    editor::Editor,
    tui::{rect::Bottom, Color, Frame, Rect, Render, Style, Text},
};

/// An [`Editor`] which can be [`Render`]ed.
///
/// [`Editor`]: EditorInternal
pub struct EditorView {}

impl EditorView {
    /// Creates a new [`EditorView`].
    pub fn new() -> Self {
        Self {}
    }

    /// Initializes the [`EditorView`] with an [`Editor`], allowing it to be [`Render`]ed.
    pub fn with_editor<'a>(&self, editor: &'a Editor) -> EditorInitialized<'a, '_> {
        EditorInitialized {
            status_bar: StatusBar {
                position: editor.selected_pos(),
            },
            edit_area: EditArea { editor },
            main_editor: self,
        }
    }
}

/// An [`EditorView`] which has been initialized by an [`Editor`].
/// This allows it to be [`Render`]ed.
pub struct EditorInitialized<'a, 'b> {
    /// The editor area.
    ///
    /// This represents the region of the screen where the editor itself is drawn.
    edit_area: EditArea<'a>,
    /// The main editor view struct.
    ///
    /// This contains all view information that persists between renders.
    main_editor: &'b EditorView,
    /// The bottom status bar of the editor.
    status_bar: StatusBar,
}

impl EditorInitialized<'_, '_> {
    /// Returns the position of the cursor in the file.
    pub fn selected_pos(&self) -> (u16, u16) {
        (
            self.status_bar.position.0 as u16,
            self.status_bar.position.1 as u16,
        )
    }
}

impl Render for EditorInitialized<'_, '_> {
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
    editor: &'a Editor,
}

impl Deref for EditArea<'_> {
    type Target = Editor;

    fn deref(&self) -> &Self::Target {
        self.editor
    }
}

impl Render for EditArea<'_> {
    fn render(&self, frame: &mut Frame, region: Rect) {
        // TODO: This really needs to be redone
        let lines: Vec<_> = self
            .editor
            .lines()
            .map(|slice| {
                let slice = slice;
                if slice.len_chars() > 0 && slice.char(slice.len_chars() - 1) == '\n' {
                    slice.slice(..slice.len_chars() - 1).to_string()
                } else {
                    slice.to_string()
                }
            })
            .collect();
        let text = Text::from(lines.as_ref());
        frame.render(&text, region);
    }
}

/// Placeholder struct for the bottom status bar of the editor.
#[derive(Debug, Default)]
struct StatusBar {
    /// The position in the file (row, column); zero-indexed.
    position: (usize, usize),
}

impl Render for StatusBar {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let bottom = region.top + region.height - 1;
        frame.set_style(Style::default().fg(Color::Black).bg(Color::White), region);
        let position = format!("{}:{}", self.position.1 + 1, self.position.0 + 1);
        for (x, c) in position.chars().enumerate() {
            frame.set_char(c, region.width - 15 + x as u16, bottom)
        }
    }
}
