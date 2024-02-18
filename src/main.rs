use anyhow;
use crossterm::{
    event::read,
    terminal::{self, EnterAlternateScreen},
};
use std::io::stdout;
fn main() -> anyhow::Result<()> {
    let mut stdout = stdout;
    terminal::enable_raw_mode()?;

    stdout.excute(EnterAlternateScreen)?;

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    read()?;

    stdout.execute(terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;
    Ok(())
}
