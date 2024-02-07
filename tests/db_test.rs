use std::error::Error;

use sqlite_rs::table::{Table, Row};
use sqlite_rs::sql::{prepare_statement, execute_statement, PrepareResult, ExecuteResult, Statement, StatementType};

fn insert_row(table: &mut Table) -> Result<(), Box<dyn Error>> {
    let mut statement = Statement {
        statement_type: StatementType::Insert,
        row_to_insert: Row::new(),
    };
    let cmd = "insert 1 user1 user1@email.com";
    let prepare_result = prepare_statement(cmd, &mut statement);
    let execute_result = execute_statement(statement, table);

    if prepare_result == PrepareResult::PrepareSuccess && execute_result == ExecuteResult::ExecuteSuccess {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to INSERT row")))
    }
}

#[test]
fn insert_and_select() {
    let mut table = Table::new();
    let mut statement = Statement {
        statement_type: StatementType::Insert,
        row_to_insert: Row::new(),
    };
    let cmd = "insert 1 user1 user1@email.com";
    let prepare_result = prepare_statement(cmd, &mut statement);
    assert_eq!(prepare_result, PrepareResult::PrepareSuccess);
    let execute_result = execute_statement(statement, &mut table);
    assert_eq!(execute_result, ExecuteResult::ExecuteSuccess);
    assert_eq!(table.num_rows, 1);

    let mut statement = Statement {
        statement_type: StatementType::Select,
        row_to_insert: Row::new(),
    };
    let cmd = "select";
    let prepare_result = prepare_statement(cmd, &mut statement);
    assert_eq!(prepare_result, PrepareResult::PrepareSuccess);
    let execute_result = execute_statement(statement, &mut table);
    assert_eq!(execute_result, ExecuteResult::ExecuteSuccess);
}

#[test]
fn insert_max_rows() {
    // max pages = 100
    // max rows per page = 4096 / 292 = 14
    // max rows = 100 * 14 = 1400
    let mut table = Table::new();
    for _i in 0..1400 {
        insert_row(&mut table).unwrap();
    }
    assert_eq!(table.num_rows, 1400);

    insert_row(&mut table).unwrap_err();
    assert_eq!(table.num_rows, 1400);
} 

#[test]
fn insert_max_len_strings() {
    let mut table = Table::new();
    let mut statement = Statement {
        statement_type: StatementType::Insert,
        row_to_insert: Row::new(),
    };
    let cmd = format!("insert 1 {} {}", "a".repeat(32), "b".repeat(255));
    let prepare_result = prepare_statement(cmd.as_str(), &mut statement);
    assert_eq!(prepare_result, PrepareResult::PrepareSuccess);
    let execute_result = execute_statement(statement, &mut table);
    assert_eq!(execute_result, ExecuteResult::ExecuteSuccess);
    assert_eq!(table.num_rows, 1);
}

#[test]
fn test_overflow_string_insert() {
    let mut statement = Statement {
        statement_type: StatementType::Insert,
        row_to_insert: Row::new(),
    };
    let cmd = format!("insert 1 {} {}", "a".repeat(33), "b".repeat(256));
    let prepare_result = prepare_statement(cmd.as_str(), &mut statement);
    assert_eq!(prepare_result, PrepareResult::PrepareStringTooLong);
}

#[test]
fn test_negative_id_insert() {
    let mut statement = Statement {
        statement_type: StatementType::Insert,
        row_to_insert: Row::new(),
    };
    let cmd = "insert -1 user1 user@email.com";
    let prepare_result = prepare_statement(cmd, &mut statement);
    assert_eq!(prepare_result, PrepareResult::PrepareNegativeID);
}