use std::{path::PathBuf, time::Duration};

use deadpool_postgres::{
    Manager, ManagerConfig, Pool, RecyclingMethod::Fast, Runtime::Tokio1, Timeouts,
};
use tokio_postgres::{Config, NoTls};
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
    let id = Uuid::new_v4().to_string();
    let _ = add_creds(
        config_dir_path,
        config_file_path,
        PgConnect {
            id: id.clone(),
            name,
            username: user,
            password,
            host,
            port,
            dbname,
        },
    )
    .map_err(|e| format!("Error adding credentials to config: {}", e))?;
    Ok((pool, id))
}
