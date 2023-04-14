//! All the code relating to the [`Editor`] lives here.

use std::io;

/// Placeholder struct for the whole editor.
#[derive(Debug, Default)]
pub struct Editor {
    /// The region of the terminal where the editing actually takes place.
    lines: Vec<String>,
    /// The file being operated on.
    file: String,
    /// Current position of the cursor
    cursor_pos: (u16, u16),
}

impl Editor {
    /// Append a single character to the [`Editor`].
    pub fn push(&mut self, c: char) {
        if c == '\n' {
            self.lines.push(String::new());
        } else {
            match self.lines.last_mut() {
                Some(last_line) => last_line.push(c),
                None => self.lines.push(String::from(c)),
            }
        }
    }

    /// Remove the last character in the [`Editor`].
    pub fn backspace(&mut self) {
        if let Some(line) = self.lines.last_mut() {
            if !line.is_empty() {
                line.pop();
            } else {
                self.lines.pop();
            }
        }
    }

    /// Open a file and read its contents to the buffer.
    pub fn open(fname: &str) -> io::Result<Self> {
        let file = std::fs::read_to_string(fname)?;
        Ok(Self {
            lines: file.lines().map(ToOwned::to_owned).collect(),
            file: fname.into(),
            cursor_pos: (0, 0),
        })
    }

    /// Write the current contents of the buffer to the file it came from.
    pub fn write(&self) -> io::Result<()> {
        std::fs::write(&self.file, self.to_string())?;
        Ok(())
    }

    /// Returns a reference to the lines of this [`Editor`].
    pub fn lines(&self) -> &[String] {
        self.lines.as_ref()
    }

    /// Returns the cursor pos of this [`Editor`].
    pub fn cursor_pos(&self) -> (u16, u16) {
        self.cursor_pos
    }
}

impl ToString for Editor {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}
