//! All the code relating to the [Editor] lives here.
//!
//! This includes all the components of the view of an editor.
//! Right now this is only the [Editor] itself and the [StatusLine].
//! These are only placeholder structs currently.

use crate::tui::{Frame, Rect, Render};

/// Placeholder struct for the whole editor.
#[derive(Debug, Default)]
pub struct Editor {
    /// The status bar at the bottom of the editor area.
    status_bar: StatusBar,
    /// The region of the terminal where the editing actually takes place.
    edit_area: EditArea,
}

impl Render for Editor {
    fn render(&self, frame: &mut Frame, region: Rect) -> std::io::Result<()> {
        frame.clear();
        let s = StatusBar {};
        s.render(frame, frame.size())?;
        Ok(())
    }
}

/// Placeholder struct for the bottom status bar of the editor.
#[derive(Debug, Default)]
struct StatusBar {}

impl Render for StatusBar {
    fn render(&self, frame: &mut Frame, region: Rect) -> std::io::Result<()> {
        // let bottom = region.height - 1;
        // for x in 0..region.width {
        //     frame.set_char('a', x, bottom);
        // }
        Ok(())
    }
}

/// The area where the editing happens.
#[derive(Debug, Default)]
struct EditArea {}
