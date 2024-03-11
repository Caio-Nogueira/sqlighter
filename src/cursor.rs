use std::error::Error;

use crate::table::{Row, Table};
use crate::btree::{Cell, Node, NodeType};
use crate::utils::binary_search_key;

pub struct Cursor<'a> {
    pub table: &'a mut Table,
    pub page_num: u32,
    pub cell_num: u32, 
    pub end_of_table: bool,
}


pub fn table_start(table: &mut Table) -> Cursor {
    let page_num = table.root_page_num;
    let root = table.pager.get_page(table.root_page_num).unwrap();

    let num_cells = root.content_len;
    
    Cursor {
        table, 
        page_num,
        cell_num: 0,
        end_of_table: num_cells == 0,
    }
}

// pub fn table_end(table: &mut Table) -> Cursor {
//     let page_num = table.root_page_num; 
//     let root = table.pager.get_page(table.root_page_num).unwrap();
//     let num_cells = root.content_len; 

//     Cursor {
//         table,
//         page_num,
//         cell_num: num_cells,
//         end_of_table: num_cells == ROWS_PER_PAGE,
//     }
// }

pub fn cursor_page<'a>(cursor: &'a mut Cursor) -> Result<&'a mut Node, Box<dyn Error>> {
    // current page pointed to by cursor
    cursor.table.pager.get_page(cursor.page_num) 
} 

pub fn cursor_value<'a>(cursor: &'a mut Cursor) -> Option<Cell> {
    // current value pointed to by cursor
    if let Ok(node) = cursor_page(cursor) {
        
        match node.clone().node_type {
            NodeType::NodeLeaf(cells) => {
                if cursor.cell_num >= cells.len() as u32 {
                    return None;
                }

                let cell = cells[cursor.cell_num as usize].clone();
                return Some(cell)
            },
            NodeType::NodeInternal(_)=> {
                return None 
            }
        }
    }
    
    None
}

pub fn table_find(table: &mut Table, key: u32) -> Result<Cursor, Box<dyn Error>> {
    let mut cursor = table_start(table);
    let root = cursor_page(&mut cursor)?;
    match root.clone().node_type {
        NodeType::NodeLeaf(cells) => {

            let idx = binary_search_key(cells.clone(), key); 
            cursor.cell_num = idx;
            if idx < cells.len() as u32 && cells[idx as usize].key == key {
                eprintln!("Key already exists");
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Key already exists")));
            }
            return Ok(cursor);
        },
        NodeType::NodeInternal(_) => unimplemented!("Internal nodes not implemented")

    }
}



pub fn cursor_advance(cursor: &mut Cursor) {
    cursor.cell_num += 1;
    let n = cursor.cell_num;
    if let Ok(node) = cursor_page(cursor) {
        
        if n >= node.content_len {
            cursor.end_of_table = true;
        }
    } 
}

pub fn cursor_insert(cursor: &mut Cursor, row: Row) -> Result<(), Box<dyn Error>> {

    let cell = Cell::new(row);
    let node = cursor_page(cursor)?;
    node.insert_cell(cell)
}