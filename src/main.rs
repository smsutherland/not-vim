use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use term_buffer::Terminal;

mod term_buffer;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut term = Terminal::new(stdout)?;

    let mut i = 0;
    term.set_cursor(i)?;

    loop {
        term.draw()?;

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

            if let KeyCode::Left = event.code {
                if i != 0 {
                    i -= 1;
                    term.set_cursor(i)?;
                }
            }
            if let KeyCode::Right = event.code {
                i += 1;
                term.set_cursor(i)?;
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
