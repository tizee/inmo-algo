use std::fmt::Display;

// Table layout
pub struct Table {
    column_size: usize,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    fn new(headers: Vec<String>) -> Self {
        Table {
            column_size: headers.len(),
            headers: headers.clone(),
            rows: vec![],
        }
    }
    fn add_row(&mut self, row: Vec<String>) -> &mut Self {
        assert!(row.len() == self.column_size);
        self.rows.push(row.clone());
        self
    }
}

pub struct TableSymobls {
    // common
    pub vertical: char,
    pub horizontal: char,
    pub left_cross: char,
    pub right_cross: char,
    // header
    pub header_left: char,
    pub header_cross: char,
    pub header_right: char,
    // row
    pub row_cross: char,
    // footer
    pub footer_left: char,
    pub footer_cross: char,
    pub footer_right: char,
}

impl Default for TableSymobls {
    fn default() -> Self {
        TableSymobls {
            vertical: '┃',
            horizontal: '━',
            left_cross: '┣',
            right_cross: '┫',
            header_left: '┏',
            header_right: '┓',
            header_cross: '┳',
            row_cross: '╋',
            footer_left: '┗',
            footer_right: '┛',
            footer_cross: '┻',
        }
    }
}

#[derive(Debug)]
enum RowLevel {
    Header,
    Row,
    Footer,
}

impl TableSymobls {
    fn padding_right(&self, name: &str, size: usize) -> String {
        assert!(name.len() <= size);
        return name.to_owned() + &" ".repeat(size - name.len());
    }
    pub fn draw_top(&self, widths: &Vec<usize>) -> String {
        let mut top: Vec<String> = vec![];
        top.push(self.header_left.to_string());
        for (i, width) in widths.iter().enumerate() {
            if i > 0 {
                top.push(self.header_cross.to_string());
            }
            top.push(self.horizontal.to_string().repeat(*width));
        }
        top.push(self.header_right.to_string());
        return top.join("");
    }
    pub fn draw_row_top(&self, widths: &Vec<usize>) -> String {
        let mut top: Vec<String> = vec![];
        top.push(self.left_cross.to_string());
        // header data
        for (i, width) in widths.iter().enumerate() {
            if i > 0 {
                top.push(self.row_cross.to_string());
            }
            top.push(self.horizontal.to_string().repeat(*width));
        }
        top.push(self.right_cross.to_string());
        return top.join("");
    }

    pub fn draw_row_content<T: AsRef<str>>(
        &self,
        widths: &Vec<usize>,
        cols: &[T],
    ) -> String {
        let mut top: Vec<String> = vec![];
        top.push(self.vertical.to_string());
        // header data
        for (i, name) in cols.iter().enumerate() {
            let name = name.as_ref();
            if i > 0 {
                top.push(self.vertical.to_string());
            }
            top.push(self.padding_right(name, widths[i]));
        }
        top.push(self.vertical.to_string());
        return top.join("");
    }

    // table bottom
    pub fn draw_bottom(&self, widths: &Vec<usize>) -> String {
        let mut top: Vec<String> = vec![];
        top.push(self.footer_left.to_string());
        for (i, width) in widths.iter().enumerate() {
            if i > 0 {
                top.push(self.footer_cross.to_string());
            }
            top.push(self.horizontal.to_string().repeat(*width));
        }
        top.push(self.footer_right.to_string());
        return top.join("");
    }
}

impl Table {
    pub fn max_widths(&self) -> Vec<usize> {
        // column width should be determined by the longest title in
        // calculate the width of each column
        let mut max_widths = vec![0; self.column_size];
        for row in self.rows.iter() {
            for (i, col) in row.iter().enumerate() {
                max_widths[i] = if col.len() > max_widths[i] {
                    col.len()
                } else {
                    max_widths[i]
                };
            }
        }
        for (i, col) in self.headers.iter().enumerate() {
            max_widths[i] = if col.len() > max_widths[i] {
                col.len()
            } else {
                max_widths[i]
            };
        }
        return max_widths;
    }
    pub fn draw(&self) -> String {
        let symbols = TableSymobls::default();
        let widths = self.max_widths();
        let mut lines: Vec<String> = vec![];
        // header
        lines.push(symbols.draw_top(&widths));
        lines.push(symbols.draw_row_content(&widths, &self.headers));
        lines.push(symbols.draw_row_top(&widths));
        // rows
        for (i, row) in self.rows.iter().enumerate() {
            lines.push(symbols.draw_row_content(&widths, row));
            if i != self.rows.len() - 1 {
                lines.push(symbols.draw_row_top(&widths));
            } else {
                lines.push(symbols.draw_bottom(&widths));
            }
        }
        return lines.join("\n");
    }
}

// header top
impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.column_size == 0 {
            return Ok(());
        }
        f.write_str(&self.draw())?;
        Ok(())
    }
}

#[cfg(test)]
mod test_layout_table {
    use super::Table;
    use super::TableSymobls;
    #[test]
    fn test_header_top() {
        let symbols = TableSymobls::default();
        let widths = vec![3, 3];
        let output = symbols.draw_top(&widths);
        assert_eq!(output, "┏━━━┳━━━┓");
    }

    #[test]
    fn test_row_top() {
        let symbols = TableSymobls::default();
        let widths = vec![3, 3];
        let output = symbols.draw_row_top(&widths);
        assert_eq!(output, "┣━━━╋━━━┫");
    }

    #[test]
    fn test_row_content() {
        let symbols = TableSymobls::default();
        let widths = vec![3, 3];
        let cols = vec!["", ""];
        let output = symbols.draw_row_content(&widths, &cols);
        assert_eq!(output, "┃   ┃   ┃");
    }

    #[test]
    fn test_footer() {
        let symbols = TableSymobls::default();
        let widths = vec![3, 3];
        let output = symbols.draw_bottom(&widths);
        assert_eq!(output, "┗━━━┻━━━┛");
    }

    #[test]
    fn test_table_max_widths() {
        let mut table = Table::new(vec![
            "movie title".to_string(),
            "date".to_string(),
            "score".to_string(),
        ]);
        table.add_row(vec![
            "Once a time in the west".to_string(),
            "1969-07-04".to_string(),
            "8.5".to_string(),
        ]);
        let widths = table.max_widths();
        assert_eq!(widths, vec![23, 10, 5]);
    }

    #[test]
    fn test_table_draw() {
        let mut table = Table::new(vec![
            "movie title".to_string(),
            "date".to_string(),
            "score".to_string(),
        ]);
        table.add_row(vec![
            "Once a time in the west".to_string(),
            "1969-07-04".to_string(),
            "8.5".to_string(),
        ]);
        let output = "\n".to_string() + &table.draw();
        assert_eq!(output,r#"
┏━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━┳━━━━━┓
┃movie title            ┃date      ┃score┃
┣━━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━╋━━━━━┫
┃Once a time in the west┃1969-07-04┃8.5  ┃
┗━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━┻━━━━━┛"#);
    }
}
