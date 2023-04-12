#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unnecessary_wraps)]

//! Not Vim is, well, just that.
//!
//! I'm just messing around trying to make my own editor because learning vimscript or lua is too
//! much work. ¯\\_(ツ)_/¯

use args::Args;
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::Terminal;

mod args;
mod editor;
mod tui;

fn main() -> io::Result<()> {
    let args = Args::parse_args();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut term = Terminal::new();

    let mut editor = editor::Editor::open(&args.file)?;

    loop {
        term.resize();
        term.draw(|f| {
            f.render(&editor, f.size());
        })?;

        if let Event::Key(event) = read()? {
            if !matches!(event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                continue;
            }

            if event.modifiers == KeyModifiers::CONTROL {
                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('w') => {
                        editor.write()?;
                        continue;
                    }
                    _ => {}
                }
            }

            match event.code {
                KeyCode::Char(c) => {
                    editor.push(c);
                }
                KeyCode::Enter => {
                    editor.push('\n');
                }
                KeyCode::Backspace => {
                    editor.backspace();
                }
                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
