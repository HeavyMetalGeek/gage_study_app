use crate::Statistics;
use eframe::egui;
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Legend, Line, MarkerShape, Plot, Points};
use gage_study::dataset::DataSet;
use std::{collections::HashMap, collections::HashSet, ops::RangeInclusive};

pub enum PlotType {
    PartMeasurement,
    OperatorMeasurement,
}

pub struct StudyPlots<'a> {
    pub dataset: Option<&'a DataSet>,
    pub plot_type: PlotType,
}

impl Default for StudyPlots<'_> {
    fn default() -> Self {
        Self {
            dataset: None,
            plot_type: PlotType::PartMeasurement,
        }
    }
}

impl<'a> StudyPlots<'a> {
    pub fn name(&self) -> &'static str {
        match self.plot_type {
            PlotType::PartMeasurement => "☰ Part Measurements",
            PlotType::OperatorMeasurement => "☰ Operator Measurements",
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        dataset: &'a Option<DataSet>,
        plot_type: PlotType,
        open: &mut bool,
    ) {
        self.dataset = dataset.as_ref();
        self.plot_type = plot_type;
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(dataset) = self.dataset {
            match self.plot_type {
                PlotType::PartMeasurement => Self::part_measurement_plot(ui, dataset),
                PlotType::OperatorMeasurement => Self::operator_measurement_plot(ui, dataset),
            };
        }
    }

    fn part_measurement_plot(ui: &mut egui::Ui, dataset: &DataSet) -> egui::Response {
        // Put part ids into a vec so they can be sorted
        // Also used for x-axis labels
        let part_hs = dataset
            .parts
            .iter()
            .map(|p| p.id.to_owned())
            .collect::<HashSet<String>>();
        let mut part_vec = Vec::from_iter(part_hs);
        part_vec.sort();
        // Map part ids to sorted indices
        let part_map = part_vec
            .iter()
            .enumerate()
            .map(|(i, v)| (v.to_owned(), i))
            .collect::<HashMap<String, usize>>();
        // Create points and average points
        let mut points: Vec<[f64; 2]> = Vec::new();
        let mut avg_points: Vec<[f64; 2]> = Vec::new();
        for part in dataset.parts.iter() {
            let idx = part_map.get(&part.id).unwrap_or(&0).to_owned() + 1;
            points.extend_from_slice(
                &part
                    .values
                    .iter()
                    .map(|v| [idx as f64, *v])
                    .collect::<Vec<[f64; 2]>>(),
            );
            let avg = part.values.iter().sum::<f64>() / part.values.len() as f64;
            avg_points.push([idx as f64, avg]);
        }
        avg_points.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        // Create markers
        let markers = Points::new("part", points)
            .shape(MarkerShape::Circle)
            .radius(4.0)
            .filled(false)
            .color(egui::Color32::GRAY);
        let avg_markers = Points::new("part", avg_points.clone())
            .shape(MarkerShape::Circle)
            .radius(4.0)
            .highlight(true)
            .color(egui::Color32::BLUE);
        let avg_line = Line::new("part", avg_points)
            .color(egui::Color32::BLUE)
            .highlight(true);
        Plot::new("part_msmt")
            .legend(Legend::default())
            .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
                if x.value > 0.0 && x.value as usize - 1 < part_vec.len() {
                    let idx = x.value as usize - 1;
                    format!("Part {}", part_vec[idx])
                } else {
                    "".to_string()
                }
            })
            .set_margin_fraction(egui::Vec2 { x: 0.1, y: 0.1 })
            .show(ui, |plot_ui| {
                plot_ui.points(markers);
                plot_ui.points(avg_markers);
                plot_ui.line(avg_line);
            })
            .response
    }

    fn operator_measurement_plot(ui: &mut egui::Ui, dataset: &DataSet) -> egui::Response {
        // Put part ids into a vec so they can be sorted
        // Also used for x-axis labels
        let op_hs = dataset
            .operators
            .iter()
            .map(|p| p.id.to_owned())
            .collect::<HashSet<String>>();
        let mut op_vec = Vec::from_iter(op_hs);
        op_vec.sort();
        // Map part ids to sorted indices
        let op_map = op_vec
            .iter()
            .enumerate()
            .map(|(i, v)| (v.to_owned(), i))
            .collect::<HashMap<String, usize>>();
        // Create points and average points
        let mut boxes: Vec<BoxPlot> = Vec::new();
        for op in dataset.operators.iter() {
            let idx = op_map.get(&op.id).unwrap_or(&0).to_owned() + 1;
            let stats = Statistics::new().from_values(&op.values);
            let box_elem = BoxElem::new(
                idx as f64,
                BoxSpread::new(stats.min, stats.q1, stats.median, stats.q3, stats.max),
            )
            .name(format!("Operator {}", op.id).as_str());
            boxes.push(BoxPlot::new(format!("Operator {}", op.id), vec![box_elem]));
        }
        // Create markers
        Plot::new("operator_msmt")
            .legend(Legend::default())
            .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
                if x.value.floor() >= 1.0 && (x.value.floor() as usize - 1 < op_vec.len()) {
                    let idx = x.value.floor() as usize - 1;
                    op_vec[idx].to_string()
                } else {
                    "".to_string()
                }
            })
            .show(ui, |plot_ui| {
                for boxx in boxes.into_iter() {
                    plot_ui.box_plot(boxx);
                }
            })
            .response
    }
}
