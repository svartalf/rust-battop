#[derive(Debug, Default)]
pub struct TabBar {
    titles: Vec<String>,
    index: usize, // Currently selected tab, 0 by default
}

impl TabBar {
    pub fn new(titles: Vec<String>) -> TabBar {
        TabBar {
            titles,
            index: 0,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn titles(&self) -> &[String] {
        self.titles.as_ref()
    }
}
