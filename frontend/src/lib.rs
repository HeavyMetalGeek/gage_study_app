#![warn(clippy::all, rust_2018_idioms)]

mod anova_table;
mod app;
mod data_table;
mod gage_eval_table;
mod statistics;
mod study_plots;
mod varcomp_table;

pub use anova_table::AnovaTableView;
pub use app::GageStudyApp;
pub use data_table::DataTableView;
pub use gage_eval_table::GageEvalTableView;
pub use statistics::Statistics;
pub use study_plots::{PlotType, StudyPlots};
pub use varcomp_table::VarCompTableView;

static DEMO_DATA_A: &'static str = include_str!("../operatorA.json");
static DEMO_DATA_B: &'static str = include_str!("../operatorB.json");
static DEMO_DATA_C: &'static str = include_str!("../operatorC.json");
