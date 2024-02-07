use std::error::Error;

use crate::constants;
pub struct Table {
    pub num_rows: u32,
    pages: Vec<Vec<u8>>
}

#[derive(Debug, Clone, Copy)]
pub struct Row {
    pub id: u32,
    pub username: [u8; 32], 
    pub email: [u8; 255],
}

impl Row {
    pub fn new() -> Row {
        Row {
            id: 0,
            username: [0; 32],
            email: [0; 255],
        }
    
    }
}

pub fn serialize_row(source: &Row, dest: &mut [u8]) {
    dest[0..4].copy_from_slice(&source.id.to_le_bytes());
    dest[4..36].copy_from_slice(&source.username);
    dest[36..291].copy_from_slice(&source.email);
}

pub fn deserialize_row(source: &[u8; constants::ROW_SIZE as usize]) -> Result<Row, Box<dyn Error>> {
    let id = u32::from_le_bytes(source[0..4].try_into().unwrap());
    let mut username = [0; 32];
    username.copy_from_slice(&source[4..36]);
    let mut email = [0; 255];
    email.copy_from_slice(&source[36..291]);
    Ok(Row { id, username, email })
}

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: vec![vec![0; constants::PAGE_SIZE as usize]; constants::TABLE_MAX_PAGES as usize],
        }
    }
}

pub fn row_slot(table: &mut Table, row_num: u32) -> Option<&mut [u8]> {
    let page_no = row_num / constants::ROWS_PER_PAGE;

    if let Some(page) = table.pages.get_mut(page_no as usize) {
        let row_offset = row_num % constants::ROWS_PER_PAGE;
        let start_index = row_offset as usize * constants::ROW_SIZE as usize;
        let end_index = (row_offset + 1) as usize * constants::ROW_SIZE as usize;
        Some(&mut page[start_index..end_index])
    } else {
        None
    }
}

pub fn insert_row(table: &mut Table, row: Row) -> Result<(), Box<dyn Error>> {
    match row_slot(table, table.num_rows) {
        Some(slot) => {
            serialize_row(&row, slot);
            table.num_rows += 1;
            Ok(())
        },
        None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to INSERT row"))),
    }
}


pub fn select_all_rows(table: &mut Table) -> Result<Vec<Row>, Box<dyn Error>> {
    let mut res: Vec<Row> = Vec::new();
    
    for i in 0..table.num_rows {
        if let Some(slot) = row_slot(table, i) {
            res.push(deserialize_row(&slot.try_into().unwrap())?);
        }
    }

    Ok(res)
}