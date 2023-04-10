//! All the code relating to the [Editor] lives here.
//!
//! This includes all the components of the view of an editor.
//! Right now this is only the [Editor] itself and the [StatusLine].
//! These are only placeholder structs currently.

use crate::term_buffer::{Frame, Render};

/// Placeholder struct for the whole editor.
pub struct Editor {}

impl Render for Editor {
    fn render(&self, frame: &mut Frame) -> std::io::Result<()> {
        frame.clear();
        let s = StatusLine {};
        s.render(frame)?;
        Ok(())
    }
}

/// Placeholder struct for the bottom status bar of the editor.
struct StatusLine {}

impl Render for StatusLine {
    fn render(&self, frame: &mut Frame) -> std::io::Result<()> {
        let region = frame.region();
        let bottom = region.height - 1;
        for x in 0..region.width {
            frame.set_char('a', x, bottom);
        }
        Ok(())
    }
}