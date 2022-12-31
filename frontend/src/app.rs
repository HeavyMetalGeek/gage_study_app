use crate::{
    AnovaTableView, DataTableView, GageEvalTableView, PlotType, StudyPlots, VarCompTableView,
};
use gage_study::{anova::Anova, data::Data, dataset::DataSet, study_evaluation::StudyEvaluation};
use rfd;

pub enum Message {
    FileOpen(String, String),
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GageStudyApp {
    // Example stuff:
    label: String,
    dataset: Vec<Data>,
    concatenate_data: bool,
    tolerance: f64,
    process_variation: f64,
    refresh_plot: bool,

    // this how you opt-out of serialization of a member
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
}

impl Default for GageStudyApp {
    fn default() -> Self {
        Self {
            label: "Gage Study Analysis".to_owned(),
            dataset: Vec::new(),
            gage_dataset: None,
            message_channel: std::sync::mpsc::channel(),
            concatenate_data: false,
            tolerance: 1.0,
            process_variation: 5.15,
            refresh_plot: false,
            open_files: Vec::new(),
            anova: None,
            study_evaluation: None,
        }
    }
}

impl GageStudyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for GageStudyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        loop {
            match self.message_channel.1.try_recv() {
                Ok(msg) => {
                    match msg {
                        Message::FileOpen(n, d) => {
                            let content: Vec<Data> =
                                serde_json::from_str(&d).expect("Failed to deserialize");
                            if self.concatenate_data {
                                self.dataset.extend_from_slice(&content);
                                self.open_files.push(n);
                            } else {
                                self.dataset = content;
                                self.open_files = vec![n];
                            };
                        }
                    };
                }
                Err(_) => {
                    break;
                }
            };
        }

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("File Upload");
            ui.checkbox(&mut self.concatenate_data, "Concatenate Files");
            let open_button = ui.add(egui::Button::new("Open..."));
            ui.separator();
            ui.heading("Open Data Files: ");
            for f in self.open_files.iter() {
                ui.label(f.clone());
            }
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
                        let file_content =
                            String::from_utf8(file.read().await).expect("Could not read file");
                        message_sender
                            .send(Message::FileOpen(file.file_name(), file_content))
                            .ok();
                    }
                });
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Tolerance: ");
                ui.add(
                    egui::DragValue::new(&mut self.tolerance)
                        .speed(0.1)
                        .clamp_range(0..=99),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Process Variation: ");
                ui.add(
                    egui::DragValue::new(&mut self.process_variation)
                        .speed(0.1)
                        .clamp_range(0..=99),
                );
            });
            if ui.button("Calculate...").clicked() {
                self.gage_dataset = match self.dataset.len() {
                    len if len > 0 => Some(DataSet::from_data("ui_data", &self.dataset)),
                    _ => None,
                };
                self.anova = match &self.gage_dataset {
                    Some(gds) => Some(Anova::from_data(gds)),
                    None => None,
                };
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

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);
            ui.heading("Gage Study Analysis");
        });

        DataTableView::default().show(ctx, &self.dataset, &mut true);
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
    // this is stupid... use any executor of your choice instead
    std::thread::spawn(move || futures::executor::block_on(f));
}
#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}
