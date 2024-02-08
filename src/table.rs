use std::{collections::HashMap, error::Error, fs::OpenOptions, io::{Read, Seek, Write}, path::Path};

use crate::constants;

pub struct Pager {
    file: std::fs::File,
    file_length: u32,
    pages: HashMap<u32, Vec<u8>>
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
        Pager {
            file,
            file_length,
            pages: HashMap::new(), 
        }
    }

    pub fn get_cache_size(&self) -> usize {
        self.pages.len()
    }

    pub fn log_cache(&self) {
        for (page_num, page_data) in &self.pages {
            println!("Page {}:", page_num);
            for byte in page_data {
                print!("{:02x} ", byte);
            }
            println!();
        }
    }

    pub fn get_page(&mut self, page_num: u32) -> Option<&mut Vec<u8>> {
        if page_num >= constants::TABLE_MAX_PAGES {
            eprintln!("Tried to fetch page number out of bounds. {} > {}", page_num, constants::TABLE_MAX_PAGES);
            return None;
        }

        if self.pages.contains_key(&page_num) {
            return self.pages.get_mut(&page_num)
        }

        match self.load_page_from_disk(page_num) {
            Ok(_) => (),
            Err(_e) => {
                self.pages.insert(page_num, vec![0; constants::PAGE_SIZE as usize]);
            }
        }
        
        self.pages.get_mut(&page_num)
    }

    fn load_page_from_disk(&mut self, page_num: u32) -> Result<(), Box<dyn Error>> {
        let mut page_data = vec![0; constants::PAGE_SIZE as usize];
        let offset = page_num * constants::PAGE_SIZE;
        self.file.seek(std::io::SeekFrom::Start(offset as u64))?;
        self.file.read_exact(&mut page_data)?;
        
        self.pages.entry(page_num).or_insert(page_data);
    
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {

       for (page_num, page_data) in &self.pages {
            let offset = page_num * constants::PAGE_SIZE;
            self.file.seek(std::io::SeekFrom::Start(offset as u64))?;
            self.file.write_all(&page_data)?;
        }

        self.file.sync_all()?;
        Ok(())
    } 
}


pub struct Table {
    pub num_rows: u32,
    pub pager: Pager,
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
    pub fn db_open(path: String) -> Table {
        let pager = Pager::open(Path::new(&path));
        let num_rows = pager.file_length / constants::ROW_SIZE;
        
        Table {
            num_rows,
            pager 
        }
    }
   
    pub fn db_close(&mut self) {
        self.pager.flush().unwrap();
    }
}

pub fn row_slot(table: &mut Table, row_num: u32) -> Option<&mut [u8]> {
    let page_no = row_num / constants::ROWS_PER_PAGE;

    // if table.pager.pages.contains_key(&page_no) {
    //     // page can already be cached
    //     let page = table.pager.pages.get_mut(&page_no).unwrap();
    //     let row_offset = row_num % constants::ROWS_PER_PAGE;
    //     let start_index = row_offset as usize * constants::ROW_SIZE as usize;
    //     let end_index = (row_offset + 1) as usize * constants::ROW_SIZE as usize;
    //     Some(&mut page[start_index..end_index])
    // } 
    
     if let Some(page) = table.pager.get_page(page_no) {
        let row_offset = row_num % constants::ROWS_PER_PAGE;
        let start_index = row_offset as usize * constants::ROW_SIZE as usize;
        let end_index = (row_offset + 1) as usize * constants::ROW_SIZE as usize;
        Some(&mut page[start_index..end_index])
    }
    
    else {
        None
    }
}

pub fn insert_row(table: &mut Table, row: Row, row_num: u32) -> Result<(), Box<dyn Error>> {
    match row_slot(table, row_num) {
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