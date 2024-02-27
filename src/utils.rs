use crate::constants;

use rand::Rng;
pub type Page = [u8; constants::PAGE_SIZE as usize];

pub fn vec_to_page(v: &mut [u8]) -> Page {
    let mut page = [0; constants::PAGE_SIZE as usize];
    page[..v.len()].copy_from_slice(&v);
    page
}

pub fn generate_key() -> u32 {
    rand::thread_rng().gen()
}