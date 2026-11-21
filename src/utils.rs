use crate::app_structs::QueryTableData;
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
