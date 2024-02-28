
use crate::{btree::Cell, constants};

pub type Page = [u8; constants::PAGE_SIZE as usize];

pub fn vec_to_page(v: &mut [u8]) -> Page {
    let mut page = [0; constants::PAGE_SIZE as usize];
    page[..v.len()].copy_from_slice(&v);
    page
}

pub fn binary_search_key(cells: Vec<Cell>, key: u32) -> Option<Cell>{
    if cells.is_empty() {
        return None;
    }
    let mut left: usize = 0;
    let mut right: usize = cells.len() - 1; 
    while left <= right {
        let mid = (left + right) / 2;
        let mid_key = cells[mid as usize].key;
        if mid_key == key {
            return Some(cells[mid].clone());
        }
        if mid_key < key {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    Some(cells[left].clone())
}
