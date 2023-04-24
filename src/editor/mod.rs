//! All the code relating to the [`Editor`] lives here.

use std::io::Read;

use anyhow::Context;

/// Placeholder struct for the whole editor.
#[derive(Debug, Default)]
pub struct Editor {
    /// The region of the terminal where the editing actually takes place.
    lines: Vec<String>,
    /// The file being operated on.
    file: String,
    /// Current position of the cursor
    cursor_pos: (usize, usize),
}

impl Editor {
    /// Append a single character to the [`Editor`].
    pub fn push(&mut self, c: char) {
        let (x, y) = self.cursor_pos;
        let line = self
            .lines
            .get_mut(y)
            .expect("Cursor was on a line which doesn't exist!");
        line.insert(x, c);
        self.cursor_pos.0 += 1;
    }

    /// Remove the last character in the [`Editor`].
    pub fn backspace(&mut self) {
        let (x, y) = self.cursor_pos;
        if x == 0 {
            if y != 0 {
                let original_len = self.lines[y - 1].len();
                let extra_line = self.lines.remove(y);
                self.lines[y - 1].push_str(&extra_line);
                self.cursor_pos = (original_len, y - 1)
            }
            return;
        }
        let line = self
            .lines
            .get_mut(y)
            .expect("Cursor was on a line which doesn't exist!");
        line.remove(x - 1);
        self.cursor_pos.0 -= 1;
    }

    /// Adds a new line where the cursor is.
    pub fn newline(&mut self) {
        let (x, y) = self.cursor_pos;
        let new_text = self.lines[y][x..].to_owned();
        self.lines[y].truncate(x);
        self.lines.insert(y + 1, new_text);
        self.cursor_pos = (0, y + 1);
    }

    /// Open a file and read its contents to the buffer.
    pub fn open(fname: &str) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(fname)
            .with_context(|| format!("Opening file `{fname}` failed."))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .with_context(|| format!("Could not read file `{fname}` to string"))?;
        Ok(Self {
            lines: buf.lines().map(ToOwned::to_owned).collect(),
            file: fname.into(),
            cursor_pos: (0, 0),
        })
    }

    /// Write the current contents of the buffer to the file it came from.
    pub fn write(&self) -> anyhow::Result<()> {
        std::fs::write(&self.file, self.to_string())?;
        Ok(())
    }

    /// Returns a reference to the lines of this [`Editor`].
    pub fn lines(&self) -> &[String] {
        self.lines.as_ref()
    }

    /// Returns the cursor pos of this [`Editor`].
    pub fn cursor_pos(&self) -> (usize, usize) {
        self.cursor_pos
    }

    /// Move the cursor left by one character.
    ///
    /// Does not move the cursor beyond the end of the line.
    /// Will not wrap to the previous line if the cursor is at the start of a line.
    pub fn move_left(&mut self) {
        if self.cursor_pos.0 != 0 {
            self.cursor_pos.0 -= 1;
        }
    }

    /// Move the cursor right by one character.
    ///
    /// Does not move the cursor beyond the end of the line.
    /// Will not wrap to the previous line if the cursor is at the end of a line.
    pub fn move_right(&mut self) {
        if self.cursor_pos.0 < self.lines[self.cursor_pos.1].len() {
            self.cursor_pos.0 += 1;
        }
    }

    /// Move the cursor down by one line.
    ///
    /// If the line below is shorter than where the cursor currently is, the cursor will move back
    /// to the end of the line.
    pub fn move_down(&mut self) {
        if self.cursor_pos.1 == self.lines.len() - 1 {
            return;
        }
        self.cursor_pos.1 += 1;
        if self.cursor_pos.0 > self.lines[self.cursor_pos.1].len() {
            self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
        }
    }

    /// Move the cursor up by one line.
    ///
    /// If the line above is shorter than where the cursor currently is, the cursor will move back
    /// to the end of the line.
    pub fn move_up(&mut self) {
        if self.cursor_pos.1 != 0 {
            self.cursor_pos.1 -= 1;
            if self.cursor_pos.0 > self.lines[self.cursor_pos.1].len() {
                self.cursor_pos.0 = self.lines[self.cursor_pos.1].len();
            }
        }
    }
}

impl ToString for Editor {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}
