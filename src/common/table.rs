// generic markdown table

pub struct MdTable {
    head: Vec<TableColumn>,
    rows: Vec<TableRow>,
}

pub trait TableRowTrait {}

pub struct TableRow {}

pub struct TableColumn {
    align: TableColumnAlign,
    txt: String,
}

pub enum TableColumnAlign {
    Center,
    Left,
    Right,
}
