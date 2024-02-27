use std::{collections::HashMap, error::Error, fs::OpenOptions, io::{Read, Seek, Write}, path::Path};
use crate::{btree::{get_node, new_leaf, Node}, cursor::{cursor_advance, cursor_insert, cursor_value, table_end, table_start}, utils::{vec_to_page, Page}};
use crate::constants;

pub struct Pager {
    file: std::fs::File,
    file_length: u32,
    num_pages: u32,
    pub pages: HashMap<u32, Node>
}

impl Pager {
    pub fn open(filename: &Path) -> Pager {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();

        let file_length = file.metadata().unwrap().len() as u32;
        if file_length % constants::PAGE_SIZE != 0 {
            eprintln!("Db file is not a whole number of pages. Corrupt file.");
            std::process::exit(1);
        }

        let num_pages = file_length / constants::PAGE_SIZE;
        Pager {
            file,
            file_length,
            num_pages,
            pages: HashMap::new(), 
        }
    }

    pub fn insert_page(&mut self, page: Node) -> Result<(), Box<dyn Error>> {
        let page_num = self.num_pages;
        self.pages.insert(page_num, page);
        Ok(())
    }


    pub fn get_page(&mut self, page_num: u32) -> Result<&mut Node, Box<dyn Error>> {
        if page_num >= constants::TABLE_MAX_PAGES {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Tried to fetch page number out of bounds")))
        }

        if self.pages.contains_key(&page_num) {
            return Ok(self.pages.get_mut(&page_num).unwrap());
        }

        match self.load_page_from_disk(page_num) {
            Ok(_) => (),
            Err(_e) => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error loading page from disk. Probably fetching not existing page number.")));
            }
        }

        Ok(self.pages.get_mut(&page_num).unwrap())
    }

    fn load_page_from_disk(&mut self, page_num: u32) -> Result<(), Box<dyn Error>> {
        let mut page_data = vec![0; constants::PAGE_SIZE as usize];
        let offset = page_num * constants::PAGE_SIZE;
        self.file.seek(std::io::SeekFrom::Start(offset as u64))?;
        self.file.read_exact(&mut page_data)?;
        let page = vec_to_page(page_data.as_mut());
        
        self.pages.entry(page_num).or_insert(get_node(page)?);
    
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {

       for (page_num, page_data) in &self.pages {
            let offset = page_num * constants::PAGE_SIZE;
            self.file.seek(std::io::SeekFrom::Start(offset as u64))?;
            self.file.write_all(&mut page_data.clone().to_page())?;
        }

        self.file.sync_all()?;
        Ok(())
    } 
}


pub struct Table {
    pub pager: Pager,
    pub root_page_num: u32,
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

pub fn deserialize_row(source: &[u8]) -> Result<Row, Box<dyn Error>> {
    let id = u32::from_le_bytes(source[0..4].try_into()?);
    let mut username = [0; 32];
    username.copy_from_slice(&source[4..36]);
    let mut email = [0; 255];
    email.copy_from_slice(&source[36..291]);
    Ok(Row { id, username, email })
}


impl Table {
    pub fn db_open(path: String) -> Table {
        let mut pager = Pager::open(Path::new(&path));
        match pager.get_page(0) {
            Ok(_) => (),
            Err(_) => {
                // New db file
                let root_node = new_leaf();
                pager.pages.insert(0, root_node);
            }
        }
        
        Table {
            root_page_num: 0,
            pager 
        }
    }
   
    pub fn db_close(&mut self) {
        self.pager.flush().unwrap();
    }

}

pub fn insert_row(table: &mut Table, row: Row) -> Result<(), Box<dyn Error>> {
    // TODO: Add btree insertion logic here
    let mut cursor = table_end(table);

    cursor_insert(&mut cursor, row) 
}


pub fn select_all_rows(table: &mut Table) -> Result<Vec<Row>, Box<dyn Error>> {
    let mut res: Vec<Row> = Vec::new();
    let mut cursor = table_start(table);
    //TODO: REFACTOR THIS -> bpage structure should be used to traverse the btree
    while !cursor.end_of_table {
        match cursor_value(&mut cursor) {
            Some(slot) => {
                
                let row = deserialize_row(&slot.value[..constants::ROW_SIZE as usize])?;
                res.push(row);
            },
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to SELECT row"))),
        }
        cursor_advance(&mut cursor);
    }
    
    Ok(res)
}