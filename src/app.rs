use std::fs::File;
use std::io::{Read, Write};

use crate::feed::Feed;

pub struct App {
    pub feeds: Vec<Feed>,
    pub tick_count: u64,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            feeds: Vec::new(),
            tick_count: 0,
        };
        app.read_feeds();
        app
    }

    pub fn update_on_tick(&mut self) {
        self.tick_count += 1;
    }

    pub fn read_feeds(&mut self) {
        self.feeds.clear();
        let mut feeds_str = String::new();
        if let Ok(mut feeds_file) = File::open("feeds.txt") {
            feeds_file.read_to_string(&mut feeds_str).unwrap();
            self.feeds.extend(feeds_str.lines().filter_map(|f| {
                if f.is_empty() {
                    None
                } else {
                    Some(Feed::new(String::from(f.trim())))
                }
            }))
        }
    }

    pub fn write_feeds(&mut self) {
        if let Ok(mut feeds_file) = File::create("feeds.txt") {
            for f in self.feeds.iter() {
                _ = feeds_file.write(f.url.as_bytes()).unwrap();
                _ = feeds_file.write("\n".as_bytes())
            }
        }
    }
}
