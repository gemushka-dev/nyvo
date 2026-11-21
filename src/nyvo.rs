use crate::app_enums::{AsyncChannelResult, ConnectionMode};
use crate::app_structs::{PgConnect, QueryTableData};
use crate::ui::connection::{
    render_connection_fields, render_connection_mode, render_saved_connections,
};
use crate::ui::table::render_query_table;
use crate::utils::{execute_query, parse_simple_query};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use deadpool_postgres::Pool;
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
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub dbname: String,
    pub connections: RwLock<HashMap<String, Pool>>,
    pub saved_connections: Vec<PgConnect>,
    pub current_connect: String,
    pub log_info: String,
    pub query_table_data: QueryTableData,
    pub block_connect: bool,
}

impl NyvoApp {
    pub fn new(
        config_dir_path: PathBuf,
        config_file_path: PathBuf,
        runtime_tokio: tokio::runtime::Runtime,
        saved_connections: Vec<PgConnect>,
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
            name: String::new(),
            user: String::from("postgres"),
            password: String::new(),
            host: String::from("localhost"),
            port: String::from("5432"),
            dbname: String::from("postgres"),
            connections: RwLock::new(HashMap::new()),
            saved_connections,
            current_connect: String::new(),
            log_info: String::new(),
            query_table_data: QueryTableData {
                columns: vec![],
                rows: vec![],
            },
            block_connect: true,
        }
    }

    pub fn get_pool(&self, id: &String) -> Option<Pool> {
        let guard = self.connections.read().unwrap();
        guard.get(id).cloned()
    }

    pub fn add_pool(&self, id: String, pool: Pool) {
        let mut guard = self.connections.write().unwrap();
        guard.insert(id, pool);
    }
}

impl eframe::App for NyvoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Ok(result) = self.rx.try_recv() {
            match result {
                AsyncChannelResult::PoolResult(v) => match v {
                    Ok(pool_result) => {
                        let pg_connect = PgConnect {
                            id: pool_result.1.clone(),
                            name: self.name.clone(),
                            username: self.user.clone(),
                            password: self.password.clone(),
                            host: self.host.clone(),
                            port: self.port.clone().trim().parse().unwrap_or(5432),
                            dbname: self.dbname.clone(),
                        };
                        self.saved_connections.push(pg_connect);
                        self.add_pool(pool_result.1, pool_result.0);
                        self.log_info = String::from(format!(
                            "Successfully connected to database '{}'",
                            self.dbname
                        ));
                        self.block_connect = true;
                    }
                    Err(e) => {
                        self.block_connect = true;
                        self.log_info = String::from(format!(
                            "Could connect to database '{}' due to error: {}",
                            self.dbname, e
                        ))
                    }
                },
                AsyncChannelResult::QueryResult(v) => match v {
                    Ok(result) => {
                        self.query_table_data = parse_simple_query(&result, &mut self.log_info);
                    }
                    Err(e) => {
                        self.log_info =
                            String::from(format!("Error during executing user query: {}", e))
                    }
                },
            }
        }
        egui::Panel::left("sidebar").resizable(true).show(ui, |ui| {
            ui.set_min_width(280.0);
            ui.heading("PostgreSQL Connection");
            render_connection_mode(self, ui);
            ui.separator();
            render_connection_fields(self, ui);
            render_saved_connections(self, ui);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let available_width = ui.available_width();
            if ui.button("Run SQL").clicked() {
                execute_query(self);
            }
            ui.separator();
            ui.add(egui::TextEdit::multiline(&mut self.query).desired_width(available_width));
            ui.separator();
            render_query_table(self, ui);
            ui.label(&self.log_info);
        });
    }
}
