use crate::app_enums::AsyncChannelResult;
use crate::app_structs::QueryTableData;
use crate::db::query_sql;
use crate::nyvo::NyvoApp;

use eframe::egui;
use tokio_postgres::SimpleQueryMessage;

pub fn parse_simple_query(
    messages: &Vec<SimpleQueryMessage>,
    log_info: &mut String,
) -> QueryTableData {
    let mut columns = Vec::new();
    let mut rows = Vec::new();

    for msg in messages {
        match msg {
            SimpleQueryMessage::Row(row) => {
                if columns.is_empty() {
                    columns = row
                        .columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect();
                }
                let mut row_values = Vec::with_capacity(row.len());
                for i in 0..row.len() {
                    let val = row.get(i).unwrap_or("NULL").to_string();
                    row_values.push(val);
                }
                rows.push(row_values);
            }
            SimpleQueryMessage::RowDescription(cols) => {
                if columns.is_empty() {
                    columns = cols.iter().map(|col| col.name().to_string()).collect();
                }
            }
            SimpleQueryMessage::CommandComplete(rows_affected) => {
                *log_info = String::from(format!("Rows affected: {}", rows_affected))
            }
            _ => {}
        };
    }

    QueryTableData { columns, rows }
}

pub fn execute_query(state: &mut NyvoApp) {
    let queryc = state.query.clone();
    let txc = state.tx.clone();
    let poolc = state.get_pool(&state.current_connect);
    match poolc {
        Some(pool) => {
            state.runtime_tokio.spawn(async move {
                let result = query_sql(&pool, &queryc).await;
                let _ = txc.send(AsyncChannelResult::QueryResult(result));
            });
        }
        None => state.log_info = String::from("There is no active connect"),
    }
}
