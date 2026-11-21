use crate::nyvo::NyvoApp;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub fn render_query_table(state: &mut NyvoApp, ui: &mut egui::Ui) {
    if state.query_table_data.columns.is_empty() {
        return;
    }
    egui::ScrollArea::horizontal()
        .id_salt("sql_table_scroll_area")
        .show(ui, |ui| {
            let mut table = TableBuilder::new(ui)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .min_scrolled_height(0.0);
            for _ in state.query_table_data.columns.iter() {
                table = table.column(Column::initial(120.0).at_least(50.0).clip(true))
            }
            table
                .header(30.0, |mut header| {
                    for col_name in state.query_table_data.columns.iter() {
                        header.col(|ui| {
                            ui.strong(col_name);
                        });
                    }
                })
                .body(|body| {
                    body.rows(25.0, state.query_table_data.rows.len(), |mut row| {
                        let row_index = row.index();
                        if let Some(row_data) = state.query_table_data.rows.get(row_index) {
                            for cell_vall in row_data {
                                row.col(|ui| {
                                    ui.label(cell_vall);
                                });
                            }
                        }
                    });
                })
        });
}
