//! All the code relating to the [Editor] lives here.
//!
//! This includes all the components of the view of an editor.
//! Right now this is only the [Editor] itself and the [StatusLine].
//! These are only placeholder structs currently.

use crate::term_buffer::{Frame, Render};

/// Placeholder struct for the whole editor.
#[derive(Debug, Default)]
pub struct Editor {
    status_bar: StatusBar,
    edit_area: EditArea,
}

impl Render for Editor {
    fn render(&self, frame: &mut Frame) -> std::io::Result<()> {
        frame.clear();
        let s = StatusBar {};
        s.render(frame)?;
        Ok(())
    }
}

/// Placeholder struct for the bottom status bar of the editor.
#[derive(Debug, Default)]
struct StatusBar {}

impl Render for StatusBar {
    fn render(&self, frame: &mut Frame) -> std::io::Result<()> {
        let region = frame.region();
        let bottom = region.height - 1;
        for x in 0..region.width {
            frame.set_char('a', x, bottom);
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct EditArea {}
