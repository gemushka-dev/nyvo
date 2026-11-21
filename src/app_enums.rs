use deadpool_postgres::Pool;
use tokio_postgres::SimpleQueryMessage;

pub enum AsyncChannelResult {
    PoolResult(Result<(Pool, String), String>),
    QueryResult(Result<Vec<SimpleQueryMessage>, String>),
}

#[derive(PartialEq)]
pub enum ConnectionMode {
    Credentials,
    URL,
}
