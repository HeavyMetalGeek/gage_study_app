use gage_study::study_evaluation::StudyEvaluation;
/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GageEvalTableView<'a> {
    pub striped: bool,
    pub resizable: bool,
    pub dataset: &'a Option<StudyEvaluation>,
}

impl<'a> Default for GageEvalTableView<'a> {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            dataset: &None,
        }
    }
}

impl<'a> GageEvalTableView<'a> {
    pub fn name(&self) -> &'static str {
        "☰ Gage Evaluation Table"
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        dataset: &'a Option<StudyEvaluation>,
        open: &mut bool,
    ) {
        self.dataset = dataset;
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.style_mut().override_text_style = Some(egui::style::TextStyle::Monospace);
        use egui_extras::{Size, StripBuilder};
        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui);
                    });
                });
            });
    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let table = TableBuilder::new(ui)
            .striped(self.striped)
            .cell_layout(egui::Layout::right_to_left(egui::Align::Max))
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .min_scrolled_height(0.0);

        let proc_var = self.dataset.as_ref().map_or(5.15, |v| v.process_variation);

        table
            .header(40.0, |mut header| {
                header.col(|ui| {
                    ui.strong("\nSource");
                });
                header.col(|ui| {
                    ui.strong("\nStdDev (SD)");
                });
                header.col(|ui| {
                    ui.strong(format!("Study Var\n({} x SD)", proc_var));
                });
                header.col(|ui| {
                    ui.strong("%Study Var\n(%SV)");
                });
                header.col(|ui| {
                    ui.strong("%Tolerance\n(SV/Tol)");
                });
            })
            .body(|mut body| {
                if let Some(study) = self.dataset {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", "Total Gage R&R"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>10.6}", study.total_gagerr.stddev));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.5}",
                                study.total_gagerr.stddev * study.process_variation
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.total_gagerr.stddev / study.total_variation.stddev * 100.0
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.total_gagerr.stddev * study.process_variation
                                    / study.tolerance
                                    * 100.0
                            ));
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", "Repeatability"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>10.6}", study.total_gagerr.repeatability.stddev));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.5}",
                                study.total_gagerr.repeatability.stddev * study.process_variation
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.total_gagerr.repeatability.stddev
                                    / study.total_variation.stddev
                                    * 100.0
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.total_gagerr.repeatability.stddev * study.process_variation
                                    / study.tolerance
                                    * 100.0
                            ));
                        });
                    });
                    if study.use_interaction {
                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", "Reproducibility"));
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{:>10.6}",
                                    study.total_gagerr.reproducibility.stddev
                                ));
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{:>10.5}",
                                    study.total_gagerr.reproducibility.stddev
                                        * study.process_variation
                                ));
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{:>10.2}",
                                    study.total_gagerr.reproducibility.stddev
                                        / study.total_variation.stddev
                                        * 100.0
                                ));
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{:>10.2}",
                                    study.total_gagerr.reproducibility.stddev
                                        * study.process_variation
                                        / study.tolerance
                                        * 100.0
                                ));
                            });
                        });
                    }
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", "Part-to-Part"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>10.6}", study.part_to_part.stddev));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.5}",
                                study.part_to_part.stddev * study.process_variation
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.part_to_part.stddev / study.total_variation.stddev * 100.0
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.part_to_part.stddev * study.process_variation
                                    / study.tolerance
                                    * 100.0
                            ));
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", "Total Variation"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>10.6}", study.total_variation.stddev));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.5}",
                                study.total_variation.stddev * study.process_variation
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.total_variation.stddev / study.total_variation.stddev * 100.0
                            ));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>10.2}",
                                study.total_variation.stddev * study.process_variation
                                    / study.tolerance
                                    * 100.0
                            ));
                        });
                    });
                }
            });
    }
}
