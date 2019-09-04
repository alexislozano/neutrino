pub struct Images {
    paths: Vec<String>,
    selected: usize,
}

impl Images {
    pub fn new() -> Self {
        Self {
            paths: vec![
                "/home/alexis/Images/autumn.jpg".to_string(),
                "/home/alexis/Images/leaf.jpg".to_string(),
                "/home/alexis/Images/snow.jpg".to_string(),
            ],
            selected: 0,
        }
    }

    pub fn selected_path(&self) -> &str {
        &self.paths[self.selected]
    }

    pub fn next(&mut self) {
        self.selected = if self.selected == 2 {
            0
        } else {
            self.selected + 1
        }
    }

    pub fn previous(&mut self) {
        self.selected = if self.selected == 0 {
            2
        } else {
            self.selected - 1
        }
    }
}
