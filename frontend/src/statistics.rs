use std::cmp::PartialOrd;
use std::convert::Into;

pub struct Statistics {
    pub mean: f64,
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            mean: 0.0,
            min: 0.0,
            q1: 0.0,
            median: 0.0,
            q3: 0.0,
            max: 0.0,
        }
    }
}

impl Statistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_values<T>(mut self, values: &[T]) -> Self
    where
        T: PartialOrd + Into<f64> + Clone,
    {
        let mut stat_values = values.to_owned();
        stat_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = stat_values.len();
        if len < 2 {
            return self;
        }
        self.min = stat_values[0].clone().into();
        self.max = stat_values[len - 1].clone().into();
        self.mean = stat_values
            .iter()
            .fold(0.0, |acc, v: &T| acc + v.to_owned().into())
            / len as f64;
        self.median = if len % 2 == 0 {
            (stat_values[len / 2].clone().into() + stat_values[len / 2 + 1].clone().into()) / 2_f64
        } else {
            stat_values[len / 2].clone().into()
        };
        (self.q1, self.q3) = if len % 2 == 0 {
            let (left, right) = stat_values.split_at(len / 2 + 1);
            (
                left[left.len() / 2].clone().into(),
                right[right.len() / 2].clone().into(),
            )
        } else {
            let (left, right) = stat_values.split_at(len / 2);
            (
                left[left.len() / 2].clone().into(),
                right[right.len() / 2].clone().into(),
            )
        };
        self
    }
}
