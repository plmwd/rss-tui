use crossterm::{
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fs::File,
    io::{self, Read, Write},
    thread,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Widget},
    Terminal,
};

fn read_feeds(feeds: &mut Vec<String>) {
    feeds.clear();
    let mut feeds_str = String::new();
    if let Ok(mut feeds_file) = File::open("feeds.txt") {
        feeds_file.read_to_string(&mut feeds_str).unwrap();
        feeds.extend(feeds_str.lines().filter_map(|f| {
            if f.is_empty() {
                None
            } else {
                Some(String::from(f.trim()))
            }
        }))
    }
}

fn write_feeds(feeds: &[String]) {
    if let Ok(mut feeds_file) = File::create("feeds.txt") {
        for f in feeds.iter() {
            _ = feeds_file.write(f.as_bytes()).unwrap();
            _ = feeds_file.write("\n".as_bytes())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut feeds: Vec<String> = vec![];
    read_feeds(&mut feeds);

    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            // NOTE: why does this need as_ref()?
            let items: Vec<ListItem> = feeds.iter().map(|f| ListItem::new(f.as_ref())).collect();
            let list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");
            f.render_stateful_widget(list, size, &mut list_state);
        })?;

        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                ..
            }) => {
                let i = match list_state.selected() {
                    Some(i) => {
                        if i >= feeds.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                list_state.select(Some(i));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                ..
            }) => {
                let i = match list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            feeds.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                list_state.select(Some(i));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                ..
            }) => {
                read_feeds(&mut feeds);
                if list_state.selected().is_none() {
                    list_state.select(Some(0));
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                ..
            }) => {
                if let Some(i) = list_state.selected() {
                    feeds.remove(i);
                    write_feeds(&feeds);

                    if feeds.is_empty() {
                        list_state.select(None);
                    } else if i >= feeds.len() {
                        list_state.select(Some(feeds.len() - 1));
                    }
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
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
