use eframe::egui;
use gage_study::data::Data;
/// Shows off a table with dynamic layout
pub struct DataTableView {
    pub striped: bool,
    pub resizable: bool,
    pub dataset: Vec<Data>,
}

impl Default for DataTableView {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            dataset: Vec::new(),
        }
    }
}

impl DataTableView {
    pub fn name(&self) -> &'static str {
        "☰ Data Table"
    }

    pub fn show(&mut self, ctx: &egui::Context, dataset: &Vec<Data>, open: &mut bool) {
        self.dataset = dataset.to_owned();
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
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
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
                    ui.vertical_centered(|ui| {
                        ui.strong("Row");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("Part");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("Operator");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("Replicate");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("Measured");
                    });
                });
            })
            .body(|mut body| {
                for (idx, d) in self.dataset.iter().enumerate() {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(idx.to_string());
                        });
                        row.col(|ui| {
                            ui.label(d.part.clone());
                        });
                        row.col(|ui| {
                            ui.label(d.operator.clone());
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", d.replicate));
                        });
                        row.col(|ui| {
                            ui.label(format!("{}", d.measured));
                        });
                    });
                }
            });
    }
}
