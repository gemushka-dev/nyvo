use std::{path::PathBuf, time::Duration};

use deadpool_postgres::{
    Manager, ManagerConfig, Pool, RecyclingMethod::Fast, Runtime::Tokio1, Timeouts,
};
use sqlparser::{
    self,
    ast::{Expr, LimitClause, Offset, OffsetRows, Statement, Value, ValueWithSpan},
    dialect::PostgreSqlDialect,
    parser,
    tokenizer::Span,
};
use tokio_postgres::{Config, NoTls, SimpleQueryMessage};
use uuid::Uuid;

use crate::{app_structs::PgConnect, storage::add_creds};

pub async fn create_pool(
    config_dir_path: &mut PathBuf,
    config_file_path: &mut PathBuf,
    name: String,
    user: String,
    password: String,
    host: String,
    port: u16,
    dbname: String,
    existing_id: Option<String>,
) -> Result<(Pool, String), String> {
    let mut config = Config::new();
    config.user(&user);
    config.password(&password);
    config.host(&host);
    config.port(port);
    config.dbname(&dbname);

    let timeouts = Timeouts {
        wait: Some(Duration::from_secs(5)),
        create: Some(Duration::from_secs(10)),
        recycle: Some(Duration::from_secs(3)),
    };

    let manager_config = ManagerConfig {
        recycling_method: Fast,
    };

    let manager = Manager::from_config(config, NoTls, manager_config);

    let pool = Pool::builder(manager)
        .timeouts(timeouts)
        .max_size(16)
        .runtime(Tokio1)
        .build()
        .map_err(|e| format!("Failed to build connection pool: {}", e))?;

    let client = pool
        .get()
        .await
        .map_err(|e| format!("Database connection error: {}", e))?;
    client
        .simple_query("SELECT 1")
        .await
        .map_err(|e| format!("Database query error: {}", e))?;
    let id = match existing_id {
        Some(v) => v,
        None => {
            let new_id = Uuid::new_v4().to_string();
            let _ = add_creds(
                config_dir_path,
                config_file_path,
                PgConnect {
                    id: new_id.clone(),
                    name,
                    username: user,
                    password,
                    host,
                    port,
                    dbname,
                },
            )
            .map_err(|e| format!("Error adding credentials to config: {}", e))?;
            new_id
        }
    };

    Ok((pool, id))
}

pub async fn query_sql(pool: &Pool, raw_sql: &String) -> Result<Vec<SimpleQueryMessage>, String> {
    let client = pool
        .get()
        .await
        .map_err(|e| format!("Database connection error: {}", e))?;
    let pg_dialect = PostgreSqlDialect {};
    let mut statements = parser::Parser::parse_sql(&pg_dialect, raw_sql)
        .map_err(|e| format!("Error paginating sql: {}", e))?;
    if statements.is_empty() {
        return Ok(vec![]);
    };
    let mut last_result = Vec::new();
    for (index, stm) in &mut statements.iter_mut().enumerate() {
        if let Statement::Query(sql) = stm {
            if sql.limit_clause.is_none() {
                let limit = Some(Expr::Value(ValueWithSpan {
                    value: Value::Number("100".to_string(), false),
                    span: Span::empty(),
                }));
                let offset = Some(Offset {
                    value: Expr::Value(ValueWithSpan {
                        value: Value::Number("0".to_string(), false),
                        span: Span::empty(),
                    }),
                    rows: OffsetRows::None,
                });
                sql.limit_clause = Some(LimitClause::LimitOffset {
                    limit,
                    offset,
                    limit_by: vec![],
                });
            }
        }
        let ready_sql = stm.to_string();
        last_result = client
            .simple_query(&ready_sql)
            .await
            .map_err(|e| format!("Error during executing query {}: {}", index + 1, e))?;
    }
    return Ok(last_result);
}
