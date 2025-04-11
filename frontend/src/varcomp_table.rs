use eframe::egui;
use gage_study::study_evaluation::StudyEvaluation;
/// Shows off a table with dynamic layout
pub struct VarCompTableView<'a> {
    pub striped: bool,
    pub resizable: bool,
    pub dataset: &'a Option<StudyEvaluation>,
}

impl Default for VarCompTableView<'_> {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            dataset: &None,
        }
    }
}

impl<'a> VarCompTableView<'a> {
    pub fn name(&self) -> &'static str {
        "☰ Variance Component Table"
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
            .cell_layout(egui::Layout::right_to_left(egui::Align::Center))
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

        table
            .header(40.0, |mut header| {
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.strong("Source");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.strong("VarComp");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("%Contribution");
                        ui.strong("(of VarComp)");
                    });
                });
            })
            .body(|mut body| {
                if let Some(study) = self.dataset {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("Total Gage R&R");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.7}", study.total_gagerr.varcomp));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>6.2}",
                                study.total_gagerr.varcomp / study.total_variation.varcomp * 100.0
                            ));
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("Repeatability");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.7}", study.total_gagerr.repeatability.varcomp));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>6.2}",
                                study.total_gagerr.repeatability.varcomp
                                    / study.total_variation.varcomp
                                    * 100.0
                            ));
                        });
                    });
                    if study.use_interaction {
                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.label("Reproducibility");
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{:>9.7}",
                                    study.total_gagerr.reproducibility.varcomp
                                ));
                            });
                            row.col(|ui| {
                                ui.label(format!(
                                    "{:>6.2}",
                                    study.total_gagerr.reproducibility.varcomp
                                        / study.total_variation.varcomp
                                        * 100.0
                                ));
                            });
                        });
                    }
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("Part-to-Part");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.7}", study.part_to_part.varcomp));
                        });
                        row.col(|ui| {
                            ui.label(format!(
                                "{:>6.2}",
                                study.part_to_part.varcomp / study.total_variation.varcomp * 100.0
                            ));
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label("Total Variation");
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.7}", study.total_variation.varcomp));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>6.2}", 100.0));
                        });
                    });
                }
            });
    }
}
