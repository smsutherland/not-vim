//! All the code relating to the [`Editor`] lives here.

use buffer::Buffer;
use ropey::{iter::Lines, RopeSlice};
use std::collections::BTreeMap;

mod buffer;

/// Documents are indexed by a unique usize.
type DocumentID = usize;

/// The main editor struct.
///
/// This has all the buffers loaded, as well as information about the cursor and which buffer is
/// selected.
#[derive(Debug, Default)]
pub struct Editor {
    /// All the buffers in the editor.
    buffers: BTreeMap<DocumentID, Buffer>,
    /// Which of the buffers is currently selected.
    ///
    /// This is a key into [`buffers`].
    ///
    /// [`buffers`]: Self::buffers
    selected_buf: DocumentID,
    /// The position of the cursor, in (x, y) format.
    ///
    /// This is a position in the buffer, not necessarilly on the screen.
    selected_pos: (usize, usize),
}

impl Editor {
    /// Open a file and read its contents to the buffer.
    pub fn open(fname: &str) -> anyhow::Result<Self> {
        let mut buffers = BTreeMap::new();
        buffers.insert(0, Buffer::open(fname)?);
        Ok(Self {
            buffers,
            selected_buf: 0,
            selected_pos: (0, 0),
        })
    }

    /// Append a single character to the [`Editor`].
    pub fn push(&mut self, c: char) {
        if let Some(buf) = self.buffers.get_mut(&self.selected_buf) {
            buf.push(c, &mut self.selected_pos);
        }
    }

    /// Remove the last character in the [`Editor`].
    pub fn backspace(&mut self) {
        if let Some(buf) = self.buffers.get_mut(&self.selected_buf) {
            buf.backspace(&mut self.selected_pos);
        }
    }

    /// Adds a new line where the cursor is.
    pub fn newline(&mut self) {
        if let Some(buf) = self.buffers.get_mut(&self.selected_buf) {
            buf.newline(&mut self.selected_pos);
        }
    }

    /// Write the current contents of the buffer to the file it came from.
    pub fn write(&self) -> anyhow::Result<()> {
        self.buffers[&self.selected_buf].write()
    }

    /// Returns a reference to the lines of this [`Editor`].
    pub fn lines(&self) -> Lines {
        self.buffers[&self.selected_buf].lines()
    }

    /// Returns a reference to the whole text of this [`Editor`].
    pub fn text(&self) -> RopeSlice {
        self.buffers[&self.selected_buf].text()
    }

    /// Returns the cursor pos of this [`Editor`].
    pub fn selected_pos(&self) -> (usize, usize) {
        self.selected_pos
    }

    /// Move the cursor left by one character.
    ///
    /// Does not move the cursor beyond the end of the line.
    /// Will not wrap to the previous line if the cursor is at the start of a line.
    pub fn move_left(&mut self) {
        if self.selected_pos.0 != 0 {
            self.selected_pos.0 -= 1;
        }
    }

    /// Move the cursor right by one character.
    ///
    /// Does not move the cursor beyond the end of the line.
    /// Will not wrap to the previous line if the cursor is at the end of a line.
    pub fn move_right(&mut self) {
        if self.selected_pos.0
            < trim_newlines(
                self.lines()
                    .nth(self.selected_pos.1)
                    .expect("invalid selected position"),
            )
            .len_chars()
        {
            self.selected_pos.0 += 1;
        }
    }

    /// Move the cursor down by one line.
    ///
    /// If the line below is shorter than where the cursor currently is, the cursor will move back
    /// to the end of the line.
    pub fn move_down(&mut self) {
        if self.selected_pos.1 == self.lines().len() - 1 {
            return;
        }
        self.selected_pos.1 += 1;
        let line_len = trim_newlines(
            self.lines()
                .nth(self.selected_pos.1)
                .expect("invalid selected position"),
        )
        .len_chars();

        if self.selected_pos.0 > line_len {
            self.selected_pos.0 = line_len;
        }
    }

    /// Move the cursor up by one line.
    ///
    /// If the line above is shorter than where the cursor currently is, the cursor will move back
    /// to the end of the line.
    pub fn move_up(&mut self) {
        if self.selected_pos.1 != 0 {
            self.selected_pos.1 -= 1;
            let line_len = trim_newlines(
                self.lines()
                    .nth(self.selected_pos.1)
                    .expect("invalid selected position"),
            )
            .len_chars();
            if self.selected_pos.0 > line_len {
                self.selected_pos.0 = line_len;
            }
        }
    }
}

/// Remove the newline character(s) from the end of a [`RopeSlice`].
///
/// This is necessary because [`RopeSlice::lines`] includes the trailing newline characters.
///
/// [`RopeSlice`]: ropey::RopeSlice
/// [`RopeSlice::lines`]: ropey::RopeSlice::lines
pub fn trim_newlines(line: RopeSlice) -> RopeSlice {
    let mut num_newline_chars = 0;
    for c in line.chars_at(line.len_chars()).reversed() {
        if matches!(
            c,
            '\u{000A}'|// Line Feed
            '\u{000D}'|// Carriage Return
            '\u{000B}'|// Vertical Tab
            '\u{000C}'|// Form Feed
            '\u{0085}'|// Next Line
            '\u{2028}'|// Line Separator
            '\u{2029}' // Paragraph Separator
        ) {
            num_newline_chars += 1;
        } else {
            break;
        }
    }
    line.slice(..line.len_chars() - num_newline_chars)
}
