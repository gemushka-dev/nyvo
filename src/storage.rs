use crate::app_structs::{NyvoConfig, PgConnect};

use std::fs;
use std::io::BufWriter;
use std::path::PathBuf;

use directories::ProjectDirs;

pub fn create_configs(
    config_dir_path: &mut PathBuf,
    config_file_path: &mut PathBuf,
) -> Result<(), std::io::Error> {
    match ProjectDirs::from("com", "nyvo", "") {
        Some(dir) => {
            *config_dir_path = dir.config_dir().to_path_buf();

            if !config_dir_path.exists() {
                fs::create_dir_all(&*config_dir_path)?;
            }

            *config_file_path = config_dir_path.join("connections.json");
            if !config_file_path.exists() {
                let file = fs::File::create(&*config_file_path)?;
                let writer = BufWriter::new(file);
                let config = NyvoConfig {
                    connections: vec![],
                };
                serde_json::to_writer(writer, &config)?;
            }

            Ok(())
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not identify user system folder",
        )),
    }
}

pub fn add_creds(
    config_dir_path: &mut PathBuf,
    config_file_path: &mut PathBuf,
    connect: PgConnect,
) -> Result<(), std::io::Error> {
    if !config_dir_path.exists() || !config_file_path.exists() {
        let _ = create_configs(config_dir_path, config_file_path)?;
    }

    let raw_json = fs::read_to_string(&config_file_path)?;
    let mut config: NyvoConfig = serde_json::from_str(&raw_json)?;
    config.connections.push(connect);

    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&config_file_path, json)?;
    Ok(())
}
