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

#[derive(PartialEq)]
enum ConnectionMode {
    Credentials,
    URL,
}

struct NyvoApp {
    tx: Sender<AsyncChannelResult>,
    rx: Receiver<AsyncChannelResult>,
    runtime_tokio: tokio::runtime::Runtime,
    config_dir_path: PathBuf,
    config_file_path: PathBuf,
    query: String,
    conn_mode: ConnectionMode,
    user: String,
    password: String,
    host: String,
    port: String,
    dbname: String,
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
            query: String::new(),
            conn_mode: ConnectionMode::Credentials,
            user: String::from("postgres"),
            password: String::new(),
            host: String::from("localhost"),
            port: String::from("5432"),
            dbname: String::from("postgres"),
        }
    }
}

impl eframe::App for NyvoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("sidebar").resizable(true).show(ui, |ui| {
            ui.heading("PostgreSQL Connection");

            ui.horizontal(|ui| {
                let creds_color = if self.conn_mode == ConnectionMode::Credentials {
                    egui::Color32::from_rgb(51, 126, 164)
                } else {
                    egui::Color32::TRANSPARENT
                };
                let creds = egui::Button::new("Credentials").fill(creds_color);
                if ui.add(creds).clicked() {
                    self.conn_mode = ConnectionMode::Credentials;
                }

                let url_color = if self.conn_mode == ConnectionMode::URL {
                    egui::Color32::from_rgb(51, 126, 164)
                } else {
                    egui::Color32::TRANSPARENT
                };
                let creds = egui::Button::new("URL string").fill(url_color);
                if ui.add(creds).clicked() {
                    self.conn_mode = ConnectionMode::URL
                }
            });
            ui.separator();

            match self.conn_mode {
                ConnectionMode::Credentials => {
                    egui::Grid::new("credentials_grid")
                        .num_columns(2)
                        .spacing([8.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Host:");
                            ui.text_edit_singleline(&mut self.host);
                            ui.end_row();

                            ui.label("Port:");
                            ui.text_edit_singleline(&mut self.port);
                            ui.end_row();

                            ui.label("Database:");
                            ui.text_edit_singleline(&mut self.dbname);
                            ui.end_row();

                            ui.label("User:");
                            ui.text_edit_singleline(&mut self.user);
                            ui.end_row();

                            ui.label("Password:");
                            ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                            ui.end_row();
                        });
                    ui.add_space(6.0);
                    ui.button("Connect with Credentials");
                    ui.separator();
                    ui.heading("Database Navigator");
                    ui.label("Connect to a database to inspect tables and columns")
                }
                ConnectionMode::URL => ui.label("Soon..."),
            };
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let available_width = ui.available_width();
            ui.button("Run SQL");
            ui.separator();
            ui.add(egui::TextEdit::multiline(&mut self.query).desired_width(available_width));
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
