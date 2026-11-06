use std::time::Duration;

use deadpool_postgres::{
    Manager, ManagerConfig, Pool, RecyclingMethod::Fast, Runtime::Tokio1, Timeouts,
};
use tokio_postgres::{Config, NoTls};

pub fn create_pool(
    user: String,
    password: String,
    host: String,
    port: u16,
    dbname: String,
) -> Result<Pool, String> {
    let mut config = Config::new();
    config.user(user);
    config.password(password);
    config.host(host);
    config.port(port);
    config.dbname(dbname);

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
    let runtime_tokio = tokio::runtime::Runtime::new()
        .map_err(|e| format!("Failed to create tokio runtime for test: {}", e))?;

    runtime_tokio.block_on(async {
        let client = pool
            .get()
            .await
            .map_err(|e| format!("Database connection error: {}", e))?;
        client
            .simple_query("SELECT 1")
            .await
            .map_err(|e| format!("Database query error: {}", e))?;

        Ok::<Pool, String>(pool)
    })
}
