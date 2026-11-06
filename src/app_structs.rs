use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PgConnect {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

#[derive(Serialize, Deserialize)]
pub struct NyvoConfig {
    pub connections: Vec<PgConnect>,
}
