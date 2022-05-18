// markdown table

pub struct MdTable {
    head: Vec<TableColumn>,
    rows: Vec<Vec<String>>,
}
pub struct TableColumn {
    align: TableColumnAlign,
    txt: String,
}

impl TableColumn {
    pub fn new(txt: String, align: TableColumnAlign) -> Self {
        TableColumn { align, txt }
    }
}

pub enum TableColumnAlign {
    Center,
    Left,
    Right,
}

impl Default for TableColumnAlign {
    fn default() -> Self {
        TableColumnAlign::Center
    }
}

impl MdTable {
    pub fn new() -> Self {
        MdTable {
            head: Vec::new(),
            rows: Vec::new(),
        }
    }
    pub fn col(&mut self, col: TableColumn) -> &mut Self {
        self.head.push(col);
        self
    }
    pub fn row(&mut self, row: Vec<String>) -> &mut Self {
        self.rows.push(row);
        self
    }
    pub fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        // render table head
        let mut head = String::from("|");
        let mut head_delimiter = String::from("|");
        for col in self.head.iter() {
            head = head + &col.txt + "|";
            match col.align {
                TableColumnAlign::Left => {
                    head_delimiter =
                        head_delimiter + " :" + "-".repeat(col.txt.len()).as_str() + " |";
                }
                TableColumnAlign::Right => {
                    head_delimiter =
                        head_delimiter + " -".repeat(col.txt.len()).as_str() + ": " + "|";
                }
                TableColumnAlign::Center => {
                    head_delimiter =
                        head_delimiter + " :" + "-".repeat(col.txt.len()).as_str() + ": " + "|";
                }
            }
        }
        lines.push(head);
        lines.push(head_delimiter);
        for row_line in self.rows.iter() {
            let mut row = String::from("|");
            for col in row_line.iter() {
                row = row + col + "|";
            }
            lines.push(row);
        }
        // render rows
        lines.join("\n")
    }
}

#[cfg(test)]
mod test_table {
    use super::{MdTable, TableColumn, TableColumnAlign};
    #[test]
    fn test_render() {
        let mut table = MdTable::new();
        table.col(TableColumn::new("1".to_string(), TableColumnAlign::Left));
        table.col(TableColumn::new("2".to_string(), TableColumnAlign::Center));
        table.col(TableColumn::new("3".to_string(), TableColumnAlign::Right));
        table.row(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        table.row(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        table.row(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        table.row(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        let res = table.render();
        println!("{}", res);
    }
}
