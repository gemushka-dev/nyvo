use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use directories::ProjectDirs;
use eframe::egui;

use serde::{Deserialize, Serialize};
use std::io::BufWriter;

use std::sync::mpsc::{Receiver, Sender, channel};

use deadpool_postgres::{
    Manager, ManagerConfig, Pool, RecyclingMethod::Fast, Runtime::Tokio1, Timeouts,
};
use tokio_postgres::{Config, NoTls, SimpleQueryMessage};

enum AsyncChannelResult {
    PoolResult(Result<Pool, String>),
    QueryResult(Result<Vec<SimpleQueryMessage>, tokio_postgres::Error>),
}

struct NyvoApp {
    tx: Sender<AsyncChannelResult>,
    rx: Receiver<AsyncChannelResult>,
    runtime_tokio: tokio::runtime::Runtime,
    config_dir_path: PathBuf,
    config_file_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Connect {
    username: String,
    password: String,
    host: String,
    port: u16,
    dbname: String,
}

#[derive(Serialize, Deserialize)]
struct NyvoConfig {
    connections: Vec<Connect>,
}

impl NyvoApp {
    fn new(
        config_dir_path: PathBuf,
        config_file_path: PathBuf,
        runtime_tokio: tokio::runtime::Runtime,
    ) -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
            rx,
            runtime_tokio,
            config_dir_path,
            config_file_path,
        }
    }
}

impl eframe::App for NyvoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_menu")
            .resizable(true)
            .show(ui, |ui| ui.label("Menu with the most used buttons"));

        egui::Panel::left("sidebar").resizable(true).show(ui, |ui| {
            ui.label("How to connect + menu with active connections");
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(format!(
                "Your current dir path: {:?}\nYour current file path: {:?}",
                self.config_dir_path, self.config_file_path
            ));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    let mut config_dir_path = PathBuf::new();
    let mut config_file_path = PathBuf::new();

    match create_configs(&mut config_dir_path, &mut config_file_path) {
        Ok(_) => {}
        Err(e) => panic!("Error occured: {}", e),
    };

    let runtime_tokio = match tokio::runtime::Runtime::new() {
        Ok(v) => v,
        Err(e) => panic!("Error occured: {}", e),
    };

    eframe::run_native(
        "nyvodb",
        native_options,
        Box::new(|_c| {
            Ok(Box::new(NyvoApp::new(
                config_dir_path,
                config_file_path,
                runtime_tokio,
            )))
        }),
    )
}

fn create_pool(
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

    let pool_result = Pool::builder(manager)
        .timeouts(timeouts)
        .max_size(16)
        .runtime(Tokio1)
        .build();

    match pool_result {
        Ok(pool) => Ok(pool),
        Err(e) => Err(format!(
            "Error occured during creating pool connection: {}",
            e
        )),
    }
}

fn create_configs(
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
