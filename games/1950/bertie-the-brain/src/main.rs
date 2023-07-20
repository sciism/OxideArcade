use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph, Terminal};

fn main() -> Result<()> {
    let mut terminal: Terminal<CrosstermBackend<Stdout>> =
        setup_terminal().context("setup failed")?;

    run(&mut terminal).context("app loop failed")?;

    restore_terminal(&mut terminal).context("restore terminal failed")?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(crate::render_app)?;

        if should_quit()? {
            break;
        }
    }

    Ok(())
}

fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
    let greeting: Paragraph<'_> = Paragraph::new("BERTIE THE BRAIN (press 'q' to quit)");

    frame.render_widget(greeting, frame.size());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}
