use crate::app_enums::{AsyncChannelResult, ConnectionMode};
use crate::ui::connection::render_connection_mode;

use std::path::PathBuf;

use eframe::egui;
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct NyvoApp {
    pub tx: Sender<AsyncChannelResult>,
    pub rx: Receiver<AsyncChannelResult>,
    pub runtime_tokio: tokio::runtime::Runtime,
    pub config_dir_path: PathBuf,
    pub config_file_path: PathBuf,
    pub query: String,
    pub conn_mode: ConnectionMode,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub dbname: String,
}

impl NyvoApp {
    pub fn new(
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

            render_connection_mode(self, ui);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let available_width = ui.available_width();
            ui.button("Run SQL");
            ui.separator();
            ui.add(egui::TextEdit::multiline(&mut self.query).desired_width(available_width));
        });
    }
}
