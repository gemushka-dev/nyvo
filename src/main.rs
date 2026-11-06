mod app_enums;
mod app_structs;
mod db;
mod storage;

use app_enums::{AsyncChannelResult, ConnectionMode};
use storage::create_configs;

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};

use eframe::egui;

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
