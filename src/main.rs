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
use editor_view::Editor as EditorView;
use gag::Hold;
use std::io;
use tui::Terminal;

mod args;
mod editor;
mod editor_view;
mod tui;

/// Unit struct which, when dropped, executes LeaveAlternateScreen on stdout.
///
/// This exists so in the event of a panic, drop is still called for this and we will still leave
/// the alternate screen.
struct AlternateScreenGuard;

impl Drop for AlternateScreenGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse_args();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let _stderr_hold = Hold::stderr()?;
    let _asg = AlternateScreenGuard;

    let mut term = Terminal::new();

    let mut editor = editor::Editor::open(&args.file)?;

    loop {
        term.resize();
        term.draw(|f| {
            let editor_view = EditorView::from(&editor);
            f.render(&editor_view, f.size());
            Some(editor_view.cursor_pos())
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
                KeyCode::Left => {
                    editor.move_left();
                }
                KeyCode::Right => {
                    editor.move_right();
                }
                KeyCode::Up => {
                    editor.move_up();
                }
                KeyCode::Down => {
                    editor.move_down();
                }
                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    // Not needed because of AlternateScreenGuard.
    // execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
