pub struct Feed {
    pub url: String,
}

impl Feed {
    pub fn new(url: String) -> Feed {
        Feed { url }
    }
}
