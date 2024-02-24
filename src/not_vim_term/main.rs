#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unnecessary_wraps)]

//! Not Vim is, well, just that.
//!
//! I'm just messing around trying to make my own editor because learning vimscript or lua is too
//! much work. ¯\\_(ツ)_/¯

use anyhow::Context;
use crossterm::{
    cursor::SetCursorStyle,
    event::{read, Event, KeyEventKind},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use editor_view::EditorView;
use gag::Hold;
use not_vim::{
    args::Args,
    config::{translate_event, Message},
    editor::{Editor, Mode},
};
use std::io;
use tui::Terminal;

mod editor_view;
mod tui;

/// Unit struct which, when dropped, executes LeaveAlternateScreen on stdout.
///
/// This exists so in the event of a panic, drop is still called for this and we will still leave
/// the alternate screen.
struct AlternateScreenGuard;

impl Drop for AlternateScreenGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            SetCursorStyle::DefaultUserShape
        );
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{err:#?}");
    }
}

/// This is the main function which is extracted out for better error handling.
fn try_main() -> anyhow::Result<()> {
    let args = Args::parse_args().context("Could not parse command line arguments")?;

    enable_raw_mode().context("Failed to enter raw mode.")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    execute!(stdout, SetCursorStyle::SteadyBlock).context("Failed to set cursor style")?;
    let _stderr_hold = Hold::stderr().context("Failed to obtain hold on stderr")?;
    let _asg = AlternateScreenGuard;

    let mut term = Terminal::new();
    let editor =
        Editor::open(&args.file).context("Could not create an editor from the file given")?;
    let mut editor_view = EditorView::new(editor);

    loop {
        term.resize();
        let size = terminal::size().expect("unable to get the dimensions of the terminal");
        editor_view.resize(size);
        term.draw(|f| {
            editor_view.render(f, f.size());
            let selected_pos = editor_view.selected_pos();
            let view_pos = editor_view.view_pos();
            Some((
                selected_pos.0 - view_pos.0 as u16,
                selected_pos.1 - view_pos.1 as u16,
            ))
        })?;

        let Event::Key(event) = read().context("Could not read an event from the terminal")? else {
            continue;
        };
        if !matches!(event.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            continue;
        }

        let message = translate_event(editor_view.editor.mode, event.into());
        match message {
            Message::Quit => {
                break;
            }
            Message::Write => {
                editor_view
                    .write()
                    .with_context(|| format!("Could not write to file {}", args.file))?;
            }
            Message::Enter => editor_view.newline(),
            Message::Backspace => editor_view.backspace(),
            Message::Left => editor_view.move_left(),
            Message::Right => editor_view.move_right(),
            Message::Up => editor_view.move_up(),
            Message::Down => editor_view.move_down(),
            Message::Char(c) => editor_view.push(c),
            Message::Mode(m) => {
                editor_view.mode = m;
                match m {
                    Mode::Normal => {
                        execute!(stdout, crossterm::cursor::SetCursorStyle::SteadyBlock)?
                    }
                    Mode::Insert => execute!(stdout, crossterm::cursor::SetCursorStyle::SteadyBar)?,
                }
            }
            Message::None => {}
        }
    }

    // Not needed because of AlternateScreenGuard.
    // disable_raw_mode().context("Failed to leave raw mode")?;
    // execute!(
    //     io::stdout(),
    //     LeaveAlternateScreen,
    //     SetCursorStyle::DefaultUserShape
    // )?;

    Ok(())
}
