use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use eframe::egui;

use serde::{Deserialize, Serialize};
use std::io::BufWriter;

struct NyvoApp {
    config_dir_path: PathBuf,
    config_file_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Connect {
    username: String,
    password: String,
    host: String,
    port: usize,
}

#[derive(Serialize, Deserialize)]
struct NyvoConfig {
    connections: Vec<Connect>,
}

impl NyvoApp {
    fn new(config_dir_path: PathBuf, config_file_path: PathBuf) -> Self {
        Self {
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
        Err(e) => println!("Error occured: {}", e),
    };

    eframe::run_native(
        "nyvodb",
        native_options,
        Box::new(|_c| Ok(Box::new(NyvoApp::new(config_dir_path, config_file_path)))),
    )
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
