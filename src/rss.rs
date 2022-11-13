use rss::Channel;

pub struct Feed {
    pub url: String,
    pub channel: Option<Channel>,
}

impl Feed {
    pub fn new(url: String) -> Feed {
        Feed { url, channel: None }
    }

    pub fn is_loaded(&self) -> bool {
        self.channel == None
    }
}
