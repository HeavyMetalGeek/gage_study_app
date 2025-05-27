use crate::{
    AnovaTableView, DataTableView, EXAMPLE_CSV, EXAMPLE_JSON, GageEvalTableView, PlotType,
    StudyPlots, VarCompTableView,
};
use eframe::egui::{self, Color32, RichText};
use gage_study::{anova::Anova, data::Data, dataset::DataSet, study_evaluation::StudyEvaluation};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub enum Message {
    FileOpen(FileInfo),
    #[allow(dead_code)]
    LogFile(Vec<u8>),
}

pub struct FileInfo {
    pub name: String,
    pub content: Vec<Data>,
}

// if we add new fields, give them default values when deserializing old state
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct GageStudyApp {
    label: String,
    dataset: Vec<Data>,
    concatenate_data: bool,
    tolerance: f64,
    process_variation: f64,
    refresh_plot: bool,
    #[serde(skip)]
    gage_dataset: Option<DataSet>,
    #[serde(skip)]
    message_channel: (
        std::sync::mpsc::Sender<Message>,
        std::sync::mpsc::Receiver<Message>,
    ),
    #[serde(skip)]
    open_files: Vec<String>,
    #[serde(skip)]
    anova: Option<Anova>,
    #[serde(skip)]
    study_evaluation: Option<StudyEvaluation>,
    #[serde(skip)]
    msg: Vec<u8>,
}

impl Default for GageStudyApp {
    fn default() -> Self {
        Self {
            label: "Gage Study Analysis".to_owned(),
            dataset: Vec::new(),
            gage_dataset: None,
            message_channel: std::sync::mpsc::channel(),
            concatenate_data: true,
            tolerance: 1.0,
            process_variation: 5.15,
            refresh_plot: false,
            open_files: Vec::new(),
            anova: None,
            study_evaluation: None,
            msg: Vec::new(),
        }
    }
}

impl GageStudyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for GageStudyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        while let Ok(msg) = self.message_channel.1.try_recv() {
            match msg {
                Message::FileOpen(f) => {
                    if self.concatenate_data {
                        self.dataset.extend(f.content);
                        self.open_files.push(f.name);
                    } else {
                        self.dataset = f.content;
                        self.open_files = vec![f.name];
                    };
                }
                Message::LogFile(bytes) => {
                    self.msg = bytes;
                }
            };
        }

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // UI elements
            ui.heading("File Upload");
            ui.checkbox(&mut self.concatenate_data, "Concatenate Files");
            let open_button = ui.add(egui::Button::new("Open..."));
            let demo_button = ui.add(egui::Button::new("Load Demo Data..."));
            ui.separator();
            ui.heading("Open Data Files: ");
            for f in self.open_files.iter() {
                ui.label(f.clone());
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Tolerance: ");
                ui.add(
                    egui::DragValue::new(&mut self.tolerance)
                        .speed(0.1)
                        .range(0..=99),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Process Variation: ");
                ui.add(
                    egui::DragValue::new(&mut self.process_variation)
                        .speed(0.1)
                        .range(0..=99),
                );
            });
            ui.vertical(|ui| {
                if ui
                    .add_enabled(!self.dataset.is_empty(), egui::Button::new("Calculate..."))
                    .clicked()
                {
                    self.gage_dataset = match self.dataset.len() {
                        len if len > 0 => Some(DataSet::from_data("ui_data", &self.dataset)),
                        _ => None,
                    };
                    self.anova = self.gage_dataset.as_ref().map(Anova::from_data);
                    self.study_evaluation = match &self.anova {
                        Some(a) => Some(
                            StudyEvaluation::from_anova(a)
                                .with_tolerance(self.tolerance)
                                .with_process_variation(self.process_variation),
                        ),
                        None => None,
                    };
                }
                if ui.button("Clear data...").clicked() {
                    self.dataset.clear();
                    self.gage_dataset = None;
                    self.open_files.clear();
                    self.anova = None;
                    self.study_evaluation = None;
                }
            });
            // Event handling
            if open_button.clicked() {
                let task = rfd::AsyncFileDialog::new()
                    .add_filter("CSV files", &["csv"])
                    .add_filter("JSON files", &["json"])
                    .set_directory("/")
                    .pick_file();
                let message_sender = self.message_channel.0.clone();

                execute(async move {
                    let file = task.await;

                    if let Some(file) = file {
                        let file_content = file.read().await;
                        // TODO: This needs to be handled better
                        let file_name = file.file_name();
                        let file_ext = Path::new(&file_name).extension().unwrap().to_str().unwrap();
                        //message_sender.send(Message::LogFile(file_content)).ok();
                        match Data::from_raw(&file_content, file_ext) {
                            Ok(data) => {
                                let _ = message_sender
                                    .send(Message::FileOpen(FileInfo {
                                        name: file.file_name(),
                                        content: data,
                                    }))
                                    .map_err(|e| tracing::error!("Sender::send: {e:?}"));
                            }
                            Err(e) => {
                                tracing::error!("Data::from_raw: {e:?}");
                            }
                        };
                    }
                });
            }
            // Load in demo data
            if demo_button.clicked() {
                self.concatenate_data = true;
                let message_sender = self.message_channel.0.clone();
                execute(async move {
                    let file_content = crate::DEMO_DATA_A;
                    match Data::from_raw_json(file_content.as_bytes()) {
                        Ok(data) => {
                            let _ = message_sender
                                .send(Message::FileOpen(FileInfo {
                                    name: "OperatorA.json".to_string(),
                                    content: data,
                                }))
                                .map_err(|e| tracing::error!("Sender::send: {e:?}"));
                        }
                        Err(e) => {
                            tracing::error!("Data::from_raw_json: {e:?}");
                        }
                    };
                });
                let message_sender = self.message_channel.0.clone();
                execute(async move {
                    let file_content = crate::DEMO_DATA_B;
                    match Data::from_raw_json(file_content.as_bytes()) {
                        Ok(data) => {
                            let _ = message_sender
                                .send(Message::FileOpen(FileInfo {
                                    name: "OperatorB.json".to_string(),
                                    content: data,
                                }))
                                .map_err(|e| tracing::error!("Sender::send: {e:?}"));
                        }
                        Err(e) => {
                            tracing::error!("Data::from_raw_json: {e:?}");
                        }
                    };
                });
                let message_sender = self.message_channel.0.clone();
                execute(async move {
                    let file_content = crate::DEMO_DATA_C;
                    match Data::from_raw_json(file_content.as_bytes()) {
                        Ok(data) => {
                            let _ = message_sender
                                .send(Message::FileOpen(FileInfo {
                                    name: "OperatorC.json".to_string(),
                                    content: data,
                                }))
                                .map_err(|e| tracing::error!("Sender::send: {e:?}"));
                        }
                        Err(e) => {
                            tracing::error!("Data::from_raw_json: {e:?}");
                        }
                    };
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            ui.heading("Gage Study Analysis");
            ui.label("");
            ui.label("To process demo data:");
            ui.label("\t1. Click the \"Load Demo Data...\" button");
            ui.label("\t2. Click the \"Calculate...\" button");
            ui.label("");
            ui.label("JSON data format:");
            ui.label(
                RichText::new(EXAMPLE_JSON)
                    .monospace()
                    .color(Color32::GREEN)
                    .background_color(Color32::TRANSPARENT),
            );
            ui.label("");
            ui.label("CSV data format:");
            ui.label(
                RichText::new(EXAMPLE_CSV)
                    .monospace()
                    .color(Color32::GREEN)
                    .background_color(Color32::TRANSPARENT),
            );
            ui.label(String::from_utf8(self.msg.clone()).unwrap().as_str());
        });

        DataTableView::default().show(ctx, &self.dataset, &mut (!self.dataset.is_empty()));
        AnovaTableView::default().show(ctx, &self.anova, &mut self.anova.is_some());
        VarCompTableView::default().show(
            ctx,
            &self.study_evaluation,
            &mut self.study_evaluation.is_some(),
        );
        GageEvalTableView::default().show(
            ctx,
            &self.study_evaluation,
            &mut self.study_evaluation.is_some(),
        );
        StudyPlots::default().show(
            ctx,
            &self.gage_dataset,
            PlotType::PartMeasurement,
            &mut self.gage_dataset.is_some(),
        );
        StudyPlots::default().show(
            ctx,
            &self.gage_dataset,
            PlotType::OperatorMeasurement,
            &mut self.gage_dataset.is_some(),
        );
    }
}

use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    // TODO: make custom executor
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
