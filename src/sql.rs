use crate::table::{insert_row, select_all_rows, Row, Table};


#[derive(Debug)]
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand(String),
}

#[derive(Debug, Clone)]
pub enum StatementType {
    Insert,
    Select,
    Invalid(String)
}

#[derive(Debug, PartialEq)]
pub enum PrepareResult{
    PrepareSuccess,
    PrepareSyntaxError,
    PrepareStringTooLong,
    PrepareUnrecognizedStatement,
    PrepareNegativeID
}

#[derive(Debug, PartialEq)]
pub enum ExecuteResult {
    ExecuteSuccess,
    ExecuteFailure(String),
}


#[derive(Debug, Clone)]
pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: Row
}



pub fn prepare_statement(cmd: &str, statement: &mut Statement) -> PrepareResult {
    if cmd.starts_with("insert") {
        statement.statement_type = StatementType::Insert;
        let split_info = cmd.split_whitespace().collect::<Vec<&str>>();

        if split_info.len() != 4 {
            return PrepareResult::PrepareSyntaxError; 
        }

        statement.row_to_insert.id = match split_info[1].parse::<i32>() {
            Ok(id) => {
                if id < 0 {
                    return PrepareResult::PrepareNegativeID;
                }
                u32::try_from(id).unwrap()
            },
            Err(_) => return PrepareResult::PrepareSyntaxError,
        };

        match split_info[2].as_bytes().get(..split_info[2].len()) {
            Some(username_bytes) => {
                if username_bytes.len() > 32 {
                    return PrepareResult::PrepareStringTooLong;
                }
                statement.row_to_insert.username[..split_info[2].len()].copy_from_slice(username_bytes);
            },
            None => return PrepareResult::PrepareSyntaxError,
        }

        match split_info[3].as_bytes().get(..split_info[3].len()) {
            Some(email_bytes) => {
                if email_bytes.len() > 255 {
                    return PrepareResult::PrepareStringTooLong;

                }
                statement.row_to_insert.email[..split_info[3].len()].copy_from_slice(email_bytes);
            },
            None => return PrepareResult::PrepareSyntaxError,
        }
        PrepareResult::PrepareSuccess
    } else if cmd.starts_with("select") {
        statement.statement_type = StatementType::Select;
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PrepareUnrecognizedStatement
    }
}

pub fn execute_statement(statement: Statement, table: &mut Table) -> ExecuteResult {
    match &statement.statement_type {
        StatementType::Insert => execute_insert(statement, table),
        StatementType::Select => execute_select(table),
        _ => panic!("Invalid statement type: {:?}", statement.statement_type) 
    }
}

pub fn execute_insert(statement: Statement, table: &mut Table)  -> ExecuteResult {
    let row = statement.row_to_insert;

    match insert_row(table, row) {
        Ok(_) => {
            println!("Inserted row with id: {}", row.id);
            ExecuteResult::ExecuteSuccess
        }, 
        Err(err) => {
            println!("Execute error");
            ExecuteResult::ExecuteFailure(err.to_string())
        } 
    }
    

}

pub fn execute_select(table: &mut Table) -> ExecuteResult {
    match select_all_rows(table) {
        Ok(res) => {
            for row in res {
                println!("({}, {}, {})", row.id, std::str::from_utf8(&row.username).unwrap(), std::str::from_utf8(&row.email).unwrap());
            }
            ExecuteResult::ExecuteSuccess
        }, 
        Err(_) => ExecuteResult::ExecuteFailure("Error selecting rows".to_string()),
    }
    
}