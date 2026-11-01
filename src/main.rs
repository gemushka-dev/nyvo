use eframe::egui;

struct NyvoApp {}

impl NyvoApp {
    fn new() -> Self {
        Self {}
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
            ui.label("SQL editor + result of user query");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "nyvodb",
        native_options,
        Box::new(|_c| Ok(Box::new(NyvoApp::new()))),
    )
}
