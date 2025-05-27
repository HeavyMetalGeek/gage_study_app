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

static DEMO_DATA_A: &str = include_str!("../operatorA.json");
static DEMO_DATA_B: &str = include_str!("../operatorB.json");
static DEMO_DATA_C: &str = include_str!("../operatorC.json");
static EXAMPLE_JSON: &str = r#"
{
    "name": string,
    "part": string,
    "operator: string,
    "replicate": integer,
    "measured": float,
    "nominal": float
}"#;
static EXAMPLE_CSV: &str = r#"
name,part,operator,replicate,measured,nominal,
example A10,part 1,operator A,1,2.78,3.0,
example A11,part 1,operator A,1,2.98,3.0,
example A12,part 1,operator A,1,2.99,3.0,
"#;
