//! A buffer is a single file that is being edited.
//!
//! Multiple editors can edit the same buffer simultaneously.
//!
//! A buffer contains both the content of the buffer and the file which it refers to.

use anyhow::Context;
use ropey::{iter::Lines, Rope, RopeSlice};

/// A single buffer of text. May refer to a specific file or be a free-floating buffer.
/// See the [module] level documentation for more.
///
/// [module]: self
#[derive(Debug, Clone)]
pub struct Buffer {
    /// Text contents of the buffer represented by a [`Rope`].
    text: Rope,
    /// The path to the file on disk (if the buffer references one).
    file: Option<String>,
}

impl Buffer {
    /// Open a file and read its contents to the buffer.
    pub fn open(fname: &str) -> anyhow::Result<Self> {
        let file = std::fs::File::open(fname)
            .with_context(|| format!("Opening file `{fname}` failed."))?;
        let rope = Rope::from_reader(file)?;
        Ok(Self {
            text: rope,
            file: Some(fname.to_owned()),
        })
    }

    /// Append a single character to the [`Buffer`] at the provided coordinates.
    pub fn push(&mut self, c: char, (x, y): &mut (usize, usize)) {
        let char_idx = self.text.line_to_char(*y) + *x;
        self.text.insert_char(char_idx, c);
        *x += 1;
    }

    /// Remove the character in the [`Buffer`] right before the provided coordinates.
    pub fn backspace(&mut self, (x, y): &mut (usize, usize)) {
        if *x == 0 {
            return;
        }
        let char_idx = self.text.line_to_char(*y) + *x - 1;
        self.text.remove(char_idx..=char_idx);
        // if *x == 0 {
        //     if *y != 0 {
        //         *x = original_len;
        //         *y -= 1;
        //     }
        //     return;
        // }
        *x -= 1;
    }

    /// Adds a new line where the cursor is.
    ///
    /// This may split a line into two if the cursor is in the middle of a line.
    pub fn newline(&mut self, (x, y): &mut (usize, usize)) {
        let char_idx = self.text.line_to_char(*y) + *x;
        self.text.insert_char(char_idx, '\n');
        *x = 0;
        *y += 1;
    }

    /// Write the current contents of the buffer to the file it came from.
    pub fn write(&self) -> anyhow::Result<()> {
        if let Some(file) = &self.file {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(file)
                .with_context(|| format!("Opening file `{file}` failed."))?;
            self.text.write_to(file)?;
        }
        Ok(())
    }

    /// Returns a reference to the lines of this [`Buffer`].
    pub fn lines(&self) -> Lines {
        self.text.lines()
    }

    /// Returns a reference to all the text of this [`Buffer`].
    pub fn text(&self) -> RopeSlice {
        self.text.slice(..)
    }
}
