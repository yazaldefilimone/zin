use crossterm;
use crossterm::cursor;
use crossterm::event;
use crossterm::event::read;
use crossterm::style;
use crossterm::style::style;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use std::io::Stdout;
use std::io::Write;
use terminal::EnterAlternateScreen;
use terminal::LeaveAlternateScreen;

enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
    EnterMode(Mode),
}

enum Mode {
    Normal,
    Insert,
}

fn handle_mode(stdout: &mut Stdout , mode: &Mode, event: event::Event) -> anyhow::Result<Option<Action>> {
    match mode {
        Mode::Insert => handle_insert_mode(stdout, event),
        Mode::Normal => handle_normal_mode(stdout, event),
    }
}

fn handle_normal_mode(stdout: &mut Stdout, event: event::Event) -> anyhow::Result<Option<Action>> {
    match event {
        event::Event::Key(_event) => match _event.code {
            // exit (aka. quit)
            event::KeyCode::Char('q') => Ok(Some(Action::Quit)),
            // Movement
            event::KeyCode::Char('h') => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Left => Ok(Some(Action::MoveLeft)),
            event::KeyCode::Char('j') => Ok(Some(Action::MoveDown)),
            event::KeyCode::Down => Ok(Some(Action::MoveDown)),
            event::KeyCode::Char('k') => Ok(Some(Action::MoveUp)),
            event::KeyCode::Up => Ok(Some(Action::MoveUp)),
            event::KeyCode::Char('l') => Ok(Some(Action::MoveRight)),
            event::KeyCode::Right => Ok(Some(Action::MoveRight)),
            event::KeyCode::Char('i') => Ok(Some(Action::EnterMode(Mode::Insert))),
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}
fn handle_insert_mode(stdout: &mut Stdout, event: event::Event) -> anyhow::Result<Option<Action>> {
    match event {
     event::Event::Key(_event) => match _event.code {
        event::KeyCode::Esc => Ok(Some(Action::EnterMode(Mode::Normal))),
        event::KeyCode::Char(character) => {
            stdout.queue(style::Print(character))?;
            Ok(None)
        }
        _ => Ok(None),
     },
        _ => Ok(None),
    }
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
        if let Some(action) = handle_mode(&mut stdout, &mode, event)? {
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
                },
                Action::EnterMode(new_mode) => {
                    mode = new_mode;
                },
                _ => {}
            }
        }
    }

    stdout.execute(LeaveAlternateScreen).unwrap();

    terminal::enable_raw_mode().unwrap();

    Ok(())
}
