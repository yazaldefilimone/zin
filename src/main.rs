use crossterm;
use crossterm::cursor;
use crossterm::event;
use crossterm::event::read;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use std::io::Write;
use terminal::EnterAlternateScreen;
use terminal::LeaveAlternateScreen;

enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}

enum Mode {
    Normal,
    Insert,
}

fn handle_mode(mode: &Mode, event: event::Event) -> anyhow::Result<Option<Action>> {
    match mode {
        Mode::Insert => handle_insert_mode(event),
        Mode::Normal => handle_normal_mode(event),
    }
}

fn handle_normal_mode(event: event::Event) -> anyhow::Result<Option<Action>> {
    match event {
        event::Event::Key(event) => match event.code {
            event::KeyCode::Char('q') => Ok(Some(Action::Quit)),
            event::KeyCode::Char('h') => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Char('j') => Ok(Some(Action::MoveDown)),
            event::KeyCode::Char('k') => Ok(Some(Action::MoveUp)),
            event::KeyCode::Char('l') => Ok(Some(Action::MoveRight)),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}
fn handle_insert_mode(event: event::Event) -> anyhow::Result<Option<Action>> {
    unimplemented!("insert mode: {:?}", event)
}
fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut mode = Mode::Normal;

    terminal::enable_raw_mode().unwrap();

    stdout.execute(EnterAlternateScreen).unwrap();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    loop {
        stdout.queue(cursor::MoveTo(cursor_x, cursor_y))?;
        stdout.flush()?;
        let event = read()?;
        if let Some(action) = handle_mode(&mode, event)? {
            match action {
                Action::Quit => break,
                Action::MoveDown => {
                    cursor_y += 1;
                }
                Action::MoveUp => {
                    cursor_y = cursor_y.saturating_sub(1);
                }
                Action::MoveRight => {
                    cursor_x += 1;
                }
                Action::MoveLeft => {
                    cursor_x = cursor_x.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    stdout.execute(LeaveAlternateScreen).unwrap();

    terminal::enable_raw_mode().unwrap();

    Ok(())
}
