use crate::core::row::Row;

#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Editor {
    row: usize,
    rows: Vec<Row>,
}

impl Editor {
    pub fn insert(&mut self, c: char) {
        if c == '\n' {}
    }
}
