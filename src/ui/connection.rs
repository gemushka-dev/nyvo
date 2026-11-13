use crate::app_enums::ConnectionMode;
use crate::nyvo::NyvoApp;

use eframe::egui;

pub fn render_connection_fields(state: &mut NyvoApp, ui: &mut egui::Ui) {
    match state.conn_mode {
        ConnectionMode::Credentials => {
            egui::Grid::new("credentials_grid")
                .num_columns(2)
                .spacing([8.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Host:");
                    ui.text_edit_singleline(&mut state.host);
                    ui.end_row();

                    ui.label("Port:");
                    ui.text_edit_singleline(&mut state.port);
                    ui.end_row();

                    ui.label("Database:");
                    ui.text_edit_singleline(&mut state.dbname);
                    ui.end_row();

                    ui.label("User:");
                    ui.text_edit_singleline(&mut state.user);
                    ui.end_row();

                    ui.label("Password:");
                    ui.add(egui::TextEdit::singleline(&mut state.password).password(true));
                    ui.end_row();
                });
            ui.add_space(6.0);
            let _ = ui.button("Connect with Credentials");
            ui.separator();
            ui.heading("Database Navigator");
            ui.label("Connect to a database to inspect tables and columns")
        }
        ConnectionMode::URL => ui.label("Soon..."),
    };
}

pub fn render_connection_mode(state: &mut NyvoApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let creds_color = if state.conn_mode == ConnectionMode::Credentials {
            egui::Color32::from_rgb(51, 126, 164)
        } else {
            egui::Color32::TRANSPARENT
        };
        let creds_btn = egui::Button::new("Credentials").fill(creds_color);
        if ui.add(creds_btn).clicked() {
            state.conn_mode = ConnectionMode::Credentials;
        }

        let url_color = if state.conn_mode == ConnectionMode::URL {
            egui::Color32::from_rgb(51, 126, 164)
        } else {
            egui::Color32::TRANSPARENT
        };
        let url_btn = egui::Button::new("URL string").fill(url_color);
        if ui.add(url_btn).clicked() {
            state.conn_mode = ConnectionMode::URL
        }
    });
}
