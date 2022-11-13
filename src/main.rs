mod app;
mod event;
mod rss;

#[allow(unused_imports)]
use crate::app::App;
use crate::event::Event;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{EventListener, Key};
use std::io;
use std::sync::Arc;
use tokio::sync::Mutex;

use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(Mutex::new(App::new()));
    start_ui(app).await?;
    Ok(())
}

async fn start_ui(app: Arc<Mutex<App>>) -> Result<(), Box<dyn std::error::Error>> {
    // setup termina
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = EventListener::new(17);

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| app.render(f))?;

        match events.next()? {
            Event::Input(Key::Char('q')) => break,
            Event::Tick => app.tick(),
            _ => {}
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
