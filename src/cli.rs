use std::io::Write;
use std::io;

use crate::table;

pub struct InputBuffer {
    pub buffer: String,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
        }
    }
}

pub fn read_input(input: &mut InputBuffer) {
    input.buffer.clear();
    io::stdin().read_line(&mut input.buffer).unwrap();
    input.buffer.pop(); // remove the trailing newline
}

pub fn print_help() {
    // print help message based on sqlite
    println!("Welcome to the monitor.  Commands end with ;");
    println!(".help             Show this message");
    println!(".exit             Exit this program");
}

/// .
///
/// # Panics
///
/// Panics if .
pub fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

pub fn log_debug(table: &table::Table) {
    println!("Num rows: {:?}", table.pager.pages.len());
}
