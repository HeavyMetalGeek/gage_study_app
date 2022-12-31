use gage_study::anova::Anova;
/// Shows off a table with dynamic layout
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AnovaTableView<'a> {
    pub striped: bool,
    pub resizable: bool,
    pub dataset: &'a Option<Anova>,
}

impl<'a> Default for AnovaTableView<'a> {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            dataset: &None,
        }
    }
}

impl<'a> AnovaTableView<'a> {
    pub fn name(&self) -> &'static str {
        "☰ Anova Table"
    }

    pub fn show(&mut self, ctx: &egui::Context, dataset: &'a Option<Anova>, open: &mut bool) {
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
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong(format!("{}", "Source"));
                });
                header.col(|ui| {
                    ui.strong(format!("{}", "df"));
                });
                header.col(|ui| {
                    ui.strong(format!("{}", "SS"));
                });
                header.col(|ui| {
                    ui.strong(format!("{}", "MS"));
                });
                header.col(|ui| {
                    ui.strong(format!("{}", "F"));
                });
                header.col(|ui| {
                    ui.strong(format!("{}", "p"));
                });
            })
            .body(|mut body| {
                if let Some(anova) = self.dataset {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{:<15}", "Part"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>3}", anova.dof_parts));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.sumsq_parts));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.meansq_parts));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.f_parts));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", "n/a"));
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{:<15}", "Operator"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>3}", anova.dof_operators));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.sumsq_operators));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.meansq_operators));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.f_operators));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", "n/a"));
                        });
                    });
                    if anova.use_interaction {
                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{:<15}", "Part*Operator"));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:>3}", anova.dof_part_operator));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:>9.5}", anova.sumsq_part_operator));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:>9.5}", anova.meansq_part_operator));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:>9.5}", anova.f_part_operator));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", ""));
                            });
                        });
                    }
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{:<15}", "Repeatability"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>3}", anova.dof_repeatability));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.sumsq_repeatability));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.meansq_repeatability));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", ""));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", ""));
                        });
                    });
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{:<15}", "Repeatability"));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>3}", anova.dof_total));
                        });
                        row.col(|ui| {
                            ui.label(format!("{:>9.5}", anova.sumsq_total));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", ""));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", ""));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", ""));
                        });
                    });
                }
            });
    }
}
