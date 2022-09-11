mod app;
mod event;
mod feed;

use crate::app::App;
use crate::event::Event;

use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{EventListener, Key};
use std::sync::Arc;
use std::{io, str};
use tokio::sync::Mutex;

use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(Mutex::new(App::new()));
    start_ui(&app).await?;
    Ok(())
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<(), Box<dyn std::error::Error>> {
    // setup termina
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let events = EventListener::new(17);

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Percentage(100)])
                .split(f.size());

            let items: Vec<ListItem> = app
                .feeds
                .iter()
                .map(|f| ListItem::new(f.url.as_ref()))
                .collect();

            let list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            let ticks = Paragraph::new(Text::from(app.tick_count.to_string()));

            f.render_widget(ticks, chunks[0]);
            f.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        match events.next()? {
            Event::Input(Key::Char('j')) => {
                let i = match list_state.selected() {
                    Some(i) => {
                        if i >= app.feeds.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                list_state.select(Some(i));
            }
            Event::Input(Key::Char('k')) => {
                let i = match list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            app.feeds.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                list_state.select(Some(i));
            }
            Event::Input(Key::Char('r')) => {
                app.read_feeds();
                if list_state.selected().is_none() {
                    list_state.select(Some(0));
                }
            }
            Event::Input(Key::Char('d')) => {
                if let Some(i) = list_state.selected() {
                    app.feeds.remove(i);
                    app.write_feeds();

                    if app.feeds.is_empty() {
                        list_state.select(None);
                    } else if i >= app.feeds.len() {
                        list_state.select(Some(app.feeds.len() - 1));
                    }
                }
            }
            Event::Input(Key::Char('q')) => break,
            Event::Tick => app.update_on_tick(),
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
