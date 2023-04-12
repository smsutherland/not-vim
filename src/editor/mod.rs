//! All the code relating to the [`Editor`] lives here.
//!
//! This includes all the components of the view of an editor.
//! Right now this is only the [`Editor`] itself and the [`StatusBar`].
//! These are only placeholder structs currently.

use std::io;

use crate::tui::{rect::Bottom, Frame, Rect, Render, Text};

/// Placeholder struct for the whole editor.
#[derive(Debug, Default)]
pub struct Editor {
    /// The status bar at the bottom of the editor area.
    _status_bar: StatusBar,
    /// The region of the terminal where the editing actually takes place.
    edit_area: EditArea,
    /// The file being operated on.
    file: String,
}

impl Editor {
    /// Append a single character to the editing area.
    pub fn push(&mut self, c: char) {
        self.edit_area.push(c);
    }

    /// Remove the last character in the [`EditArea`].
    pub fn backspace(&mut self) {
        self.edit_area.backspace();
    }

    /// Open a file and read its contents to the buffer.
    pub fn open(fname: &str) -> io::Result<Self> {
        let file = std::fs::read_to_string(fname)?;
        Ok(Self {
            _status_bar: StatusBar {},
            edit_area: EditArea {
                lines: file.lines().map(ToOwned::to_owned).collect(),
            },
            file: fname.into(),
        })
    }

    /// Write the current contents of the buffer to the file it came from.
    pub fn write(&self) -> io::Result<()> {
        std::fs::write(&self.file, self.edit_area.to_string())?;
        Ok(())
    }
}

impl Render for Editor {
    fn render(&self, frame: &mut Frame, region: Rect) {
        frame.clear();
        let s = StatusBar {};
        let regions = region.partition(Bottom);
        let bottom_bar = regions[0];
        let editor_area = regions[1];
        frame.render(&s, bottom_bar);
        frame.render(&self.edit_area, editor_area);
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

/// The area where the editing happens.
#[derive(Debug)]
struct EditArea {
    /// All the individual lines in the [`EditArea`].
    lines: Vec<String>,
}

impl Default for EditArea {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }
}

impl EditArea {
    /// Append a single character to the editing area.
    fn push(&mut self, c: char) {
        if c == '\n' {
            self.lines.push(String::new());
        } else {
            match self.lines.last_mut() {
                Some(last_line) => last_line.push(c),
                None => self.lines.push(String::from(c)),
            }
        }
    }

    /// Remove the last character in the last line of the buffer.
    /// If the last line is empty, removes the last line.
    fn backspace(&mut self) {
        if let Some(line) = self.lines.last_mut() {
            if !line.is_empty() {
                line.pop();
            } else {
                self.lines.pop();
            }
        }
    }
}

impl ToString for EditArea {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

impl Render for EditArea {
    fn render(&self, frame: &mut Frame, region: Rect) {
        let text = Text::from(self.lines.as_slice());
        frame.render(&text, region);
    }
}
