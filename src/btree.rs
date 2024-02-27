use std::error::Error;

use crate::constants;
use crate::table::{serialize_row, Row};
use crate::utils::{generate_key, Page};

#[derive(Debug, Clone)]
pub struct Cell {
    pub key: u32,
    pub value: [u8; constants::ROW_SIZE as usize],
}

impl Cell {
    pub fn new(row: Row) -> Cell {
        let mut cell = Cell {
            key: generate_key(),
            value: [0; constants::ROW_SIZE as usize],
        };
        serialize_row(&row, cell.value.as_mut());
        
        cell
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    NodeInternal(Vec<u32>), // children -> Vec with keys
    NodeLeaf(Vec<Cell>), // cells 
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub parent: u32,
    pub content_len: u32, // num of cells / children, depending on node type
}

fn get_content_len(p: Page) -> u32 {
    let mut content_len = [0u8; 4];
    content_len.copy_from_slice(&p[constants::NODE_CONTENT_LEN_OFFSET as usize..(constants::NODE_CONTENT_LEN_OFFSET + 4) as usize]);
    u32::from_ne_bytes(content_len)
}

fn set_content_len(p: &mut Page, content_len: u32) {
    p[constants::NODE_CONTENT_LEN_OFFSET as usize..(constants::NODE_CONTENT_LEN_OFFSET + 4) as usize].copy_from_slice(&content_len.to_ne_bytes());
}

fn get_node_type(p: Page) -> Result<NodeType, Box<dyn Error>>{
    match p[0] {
        0 => {
            let mut children = Vec::new();
            for i in 0..get_content_len(p) {
                let mut child = [0u8; 4];
                child.copy_from_slice(&p[(constants::NODE_CONTENT_OFFSET + i * 4) as usize..(constants::NODE_CONTENT_OFFSET + i * 4 + 4) as usize]);
                children.push(u32::from_ne_bytes(child));
            }
            Ok(NodeType::NodeInternal(children))
        },
        1 => {
            let mut cells = Vec::new();
            let mut i = constants::NODE_CONTENT_OFFSET;
            let content_len = get_content_len(p);
            for _ in 0..content_len {
                let mut key = [0u8; 4];
                key.copy_from_slice(&p[i as usize..(i + 4) as usize]);
                let key = u32::from_ne_bytes(key);
                i += 4;
                let mut value = [0u8; constants::ROW_SIZE as usize];
                value.copy_from_slice(&p[i as usize..(i + constants::ROW_SIZE) as usize]);
                i += constants::ROW_SIZE;
                cells.push(Cell { key, value });
            }
            Ok(NodeType::NodeLeaf(cells))
        },
        _ => Err("Invalid node type".into()),
    }
}

fn get_parent(p: Page) -> u32 {
    let mut parent = [0u8; 4];
    parent.copy_from_slice(&p[constants::NODE_PARENT_OFFSET as usize..(constants::NODE_PARENT_OFFSET + 4) as usize]);
    u32::from_ne_bytes(parent)
}

fn set_parent(p: &mut Page, parent: u32) {
    p[constants::NODE_PARENT_OFFSET as usize..(constants::NODE_PARENT_OFFSET + 4) as usize].copy_from_slice(&parent.to_ne_bytes());
}

pub fn get_node(p: Page) -> Result<Node, Box<dyn Error>> {
    let node_type = get_node_type(p)?;
    Ok(Node {
        node_type,
        parent: get_parent(p),
        content_len: get_content_len(p),
    })
}

pub fn init_leaf_node(p: &mut Page) {
    p[0] = 1;
    set_content_len(p, 0);
}

impl Node {
    pub fn to_page(self) -> Page {
        let mut page = [0; constants::PAGE_SIZE as usize];
        match self.node_type {
            NodeType::NodeInternal(children) => {
                page[0] = 0;
                set_content_len(&mut page, children.len() as u32);
                for (i, child) in children.iter().enumerate() {
                    page[(constants::NODE_CONTENT_OFFSET as usize + i * 4) as usize..(constants::NODE_CONTENT_OFFSET as usize + i * 4 + 4) as usize].copy_from_slice(&child.to_ne_bytes());
                }
            },
            NodeType::NodeLeaf(cells) => {
                page[0] = 1;
                set_content_len(&mut page, cells.len() as u32);
                let mut i = constants::NODE_CONTENT_OFFSET;
                for cell in cells {
                    page[i as usize..(i + 4) as usize].copy_from_slice(&cell.key.to_ne_bytes());
                    i += 4;
                    page[i as usize..(i + constants::ROW_SIZE) as usize].copy_from_slice(&cell.value);
                    i += constants::ROW_SIZE;
                }
            },
        }
        page
    }

    pub fn insert_cell(&mut self, cell: Cell) {

        match &mut self.node_type {
            NodeType::NodeInternal(children) => {
                children.push(cell.key);
                children.sort();
            },
            NodeType::NodeLeaf(cells) => {
                cells.push(cell);
                cells.sort_by(|a, b| a.key.cmp(&b.key));
            },
        }
        self.content_len += 1;
    }

}


pub fn new_leaf() -> Node {
    Node {
        node_type: NodeType::NodeLeaf(Vec::new()),
        parent: 0,
        content_len: 0,
    }
}