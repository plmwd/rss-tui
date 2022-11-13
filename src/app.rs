use rss::Item;
use tui::backend::Backend;
use tui::Frame;

use crate::rss::Feed;

pub enum Block {
    Feeds,
    Content,
}

pub struct App {
    pub feeds: Vec<Feed>,
    pub tick_count: u64,
    pub active_block: Block,
    pub selected_item: Option<Item>,
}

impl App {
    pub fn new() -> Self {
        App {
            feeds: Vec::new(),
            tick_count: 0,
            active_block: Block::Feeds,
            selected_item: None,
        }
    }

    pub fn tick(&mut self) {}

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {}
}
