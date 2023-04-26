//! A buffer is a single file that is being edited.
//!
//! Multiple editors can edit the same buffer simultaneously.
//!
//! A buffer contains both the content of the buffer and the file which it refers to.

use anyhow::Context;
use std::io::Read;

/// A single buffer of text. May refer to a specific file or be a free-floating buffer.
/// See the [module] level documentation for more.
///
/// [module]: self
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The lines of the buffer.
    lines: Vec<String>,
    /// The path to the file on disk (if the buffer references one).
    file: Option<String>,
}

impl Buffer {
    /// Open a file and read its contents to the buffer.
    pub fn open(fname: &str) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(fname)
            .with_context(|| format!("Opening file `{fname}` failed."))?;
        let mut lines = String::new();
        file.read_to_string(&mut lines)
            .with_context(|| format!("Could not read file `{fname}` to string"))?;
        Ok(Self {
            lines: lines.lines().map(ToOwned::to_owned).collect(),
            file: Some(fname.to_owned()),
        })
    }

    /// Append a single character to the [`Buffer`] at the provided coordinates.
    pub fn push(&mut self, c: char, (x, y): &mut (usize, usize)) {
        let line = self
            .lines
            .get_mut(*y)
            .expect("Cursor was on a line which doesn't exist!");
        line.insert(*x, c);
        *x += 1;
    }

    /// Remove the character in the [`Buffer`] right before the provided coordinates.
    pub fn backspace(&mut self, (x, y): &mut (usize, usize)) {
        if *x == 0 {
            if *y != 0 {
                let original_len = self.lines[*y - 1].len();
                let extra_line = self.lines.remove(*y);
                self.lines[*y - 1].push_str(&extra_line);
                *x = original_len;
                *y -= 1;
            }
            return;
        }
        let line = self
            .lines
            .get_mut(*y)
            .expect("Cursor was on a line which doesn't exist!");
        line.remove(*x - 1);
        *x -= 1;
    }

    /// Adds a new line where the cursor is.
    ///
    /// This may split a line into two if the cursor is in the middle of a line.
    pub fn newline(&mut self, (x, y): &mut (usize, usize)) {
        let new_text = self.lines[*y][*x..].to_owned();
        self.lines[*y].truncate(*x);
        self.lines.insert(*y + 1, new_text);
        *x = 0;
        *y += 1;
    }

    /// Write the current contents of the buffer to the file it came from.
    pub fn write(&self) -> anyhow::Result<()> {
        if let Some(file) = &self.file {
            std::fs::write(file, self.lines.join("\n"))?;
        }
        Ok(())
    }

    /// Returns a reference to the lines of this [`Buffer`].
    pub fn lines(&self) -> &[String] {
        self.lines.as_ref()
    }
}
