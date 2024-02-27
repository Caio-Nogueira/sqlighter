#[allow(dead_code)]
pub const PAGE_SIZE: u32 = 4096;

#[allow(dead_code)]
pub const TABLE_MAX_PAGES: u32 = 100;

#[allow(dead_code)]
pub const ROW_SIZE: u32 = 292;

#[allow(dead_code)]
pub const ID_SIZE: u32 = 4;

#[allow(dead_code)]
pub const USERNAME_SIZE: u32 = 32;

#[allow(dead_code)]
pub const EMAIL_SIZE: u32 = 255;

#[allow(dead_code)]
pub const LEAF_NODE_KEY_SIZE: u32 = 4;

#[allow(dead_code)]
pub const LEAF_NODE_VALUE_SIZE: u32 = ROW_SIZE;

#[allow(dead_code)]
pub const LEAF_NODE_CELL_SIZE: u32 = LEAF_NODE_KEY_SIZE + ROW_SIZE;

#[allow(dead_code)]
pub const NODE_TYPE_SIZE: u32 = 1;

#[allow(dead_code)]
pub const NODE_PARENT_OFFSET: u32 = NODE_TYPE_SIZE;

#[allow(dead_code)]
pub const NODE_PARENT_SIZE: u32 = 4;

#[allow(dead_code)]
pub const NODE_CONTENT_LEN_OFFSET: u32 = NODE_PARENT_OFFSET + NODE_PARENT_SIZE;

#[allow(dead_code)]
pub const NODE_CONTENT_LEN_SIZE: u32 = 4;

#[allow(dead_code)]
pub const NODE_CONTENT_OFFSET: u32 = NODE_CONTENT_LEN_OFFSET + NODE_CONTENT_LEN_SIZE;

#[allow(dead_code)]
pub const NODE_METADATA_SIZE: u32 = NODE_TYPE_SIZE + NODE_PARENT_SIZE + NODE_CONTENT_LEN_SIZE; 

#[allow(dead_code)]
pub const ROWS_PER_PAGE: u32 = (PAGE_SIZE - NODE_METADATA_SIZE)/ ROW_SIZE;

#[allow(dead_code)]
pub const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;
