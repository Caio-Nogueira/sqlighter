use crate::table::Table;


pub struct Cursor<'a> {
    pub table: &'a mut Table,
    pub row_num: u32,
    pub end_of_table: bool,
}


pub fn table_start(table: &mut Table) -> Cursor {
    let num_rows = table.num_rows;
    Cursor {
        table: table, 
        row_num: 0,
        end_of_table: num_rows == 0,
    }
}

pub fn table_end(table: &mut Table) -> Cursor {
    let num_rows = table.num_rows;
    Cursor {
        table,
        row_num: num_rows,
        end_of_table: true,
    }
}

pub fn cursor_advance(cursor: &mut Cursor) {
    cursor.row_num += 1;
    if cursor.row_num >= cursor.table.num_rows {
        cursor.end_of_table = true;
    }
}