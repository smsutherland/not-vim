#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]

//! Not Vim is, well, just that.
//!
//! I'm just messing around trying to make my own editor because learning vimscript or lua is too
//! much work. ¯\\_(ツ)_/¯

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::Terminal;

mod editor;
mod tui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut term = Terminal::new();

    let mut i = 0;
    term.set_cursor(i)?;

    let e = editor::Editor::default();

    loop {
        term.resize();
        term.draw(|f| {
            f.render(&e, f.size())?;
            Ok(())
        })?;

        if let Event::Key(event) = read()? {
            if !matches!(event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                continue;
            }
            if let KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } = event
            {
                break;
            }

            if let KeyCode::Char(c) = event.code {
                term.set(c, i);
                i += 1;
                term.set_cursor(i)?;
            }

            if event.code == KeyCode::Left && i != 0 {
                i -= 1;
                term.set_cursor(i)?;
            }
            if event.code == KeyCode::Right {
                i += 1;
                term.set_cursor(i)?;
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
