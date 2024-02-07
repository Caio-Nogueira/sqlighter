#[allow(dead_code)]
pub const PAGE_SIZE: u32 = 4096;

#[allow(dead_code)]
pub const TABLE_MAX_PAGES: u32 = 100;

#[allow(dead_code)]
pub const ROW_SIZE: u32 = 292;

#[allow(dead_code)]
pub const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;

#[allow(dead_code)]
pub const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[allow(dead_code)]
pub const ID_SIZE: u32 = 4;

#[allow(dead_code)]
pub const USERNAME_SIZE: u32 = 32;

#[allow(dead_code)]
pub const EMAIL_SIZE: u32 = 255;
