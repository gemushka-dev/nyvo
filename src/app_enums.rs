use deadpool_postgres::Pool;
use tokio_postgres::SimpleQueryMessage;

pub enum AsyncChannelResult {
    PoolResult(Result<Pool, String>),
    QueryResult(Result<Vec<SimpleQueryMessage>, tokio_postgres::Error>),
}

#[derive(PartialEq)]
pub enum ConnectionMode {
    Credentials,
    URL,
}
