#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Row {
    column: usize,
    content: Vec<String>,
}

impl Row {
    pub fn insert(&mut self, c: char) {
        self.content.insert(self.column, c.to_string());
    }

    pub fn show(&self) -> String {
        self.content.join("")
    }
}
