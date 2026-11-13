mod app_enums;
mod app_structs;
mod db;
mod nyvo;
mod storage;
mod ui;

use nyvo::NyvoApp;
use storage::create_configs;

use std::path::PathBuf;

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
