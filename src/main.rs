mod cli;
mod sql;
mod table;
mod cursor;
mod constants;
use std::env;

use crate::sql::MetaCommandResult;
use crate::sql::{prepare_statement, execute_statement, PrepareResult};
use crate::sql::Statement;

fn main() {
    cli::print_help();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <db_file>", args[0]);
        return;
    }
    let db_file = &args[1];

    let mut input = cli::InputBuffer::new();

    let mut table = table::Table::db_open(db_file.to_string());

    loop {
        cli::print_prompt();
        cli::read_input(&mut input);
        handle_statement(input.buffer.clone(), &mut table);
    }
}

pub fn handle_statement(cmd: String, table: &mut table::Table) {
    let mut statement: Statement = Statement {
        statement_type: sql::StatementType::Invalid(cmd.clone()),
        row_to_insert: table::Row::new(),
    };

    if cmd.starts_with(".") {
        match do_meta_command(cmd, table) {
            MetaCommandResult::MetaCommandSuccess => (),
            MetaCommandResult::MetaCommandUnrecognizedCommand(cmd) => {
                println!("Unrecognized command '{}'", cmd);
            }
        }
    } else {
        match prepare_statement(cmd.as_str(), &mut statement) {
            PrepareResult::PrepareSuccess =>{ 
                execute_statement(statement, table);
                ()
            },
            PrepareResult::PrepareSyntaxError => {
                println!("Syntax error. Could not parse statement '{}'", cmd)
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'", cmd)
            }
            PrepareResult::PrepareStringTooLong => {
                println!("String is too long")
            }
            PrepareResult::PrepareNegativeID => {
                println!("ID must be positive")
            }
        }
    }
}


pub fn do_meta_command(cmd: String, table: &mut table::Table) -> MetaCommandResult {
    if cmd == ".exit" {
        table.db_close();
        std::process::exit(0);
    } else if cmd == ".help" {
        cli::print_help();
        MetaCommandResult::MetaCommandSuccess
    } else if cmd == ".debug" {
        cli::log_debug(table);
        MetaCommandResult::MetaCommandSuccess
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand(cmd)
    }
}

