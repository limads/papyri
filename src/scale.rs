use std::collections::HashMap;
use super::ScaleProperty;
use super::context_mapper;
use std::str::FromStr;

/// Carries the logic of how scale adjustment to the mapping should be done.
/// Tight means the two scales will extend just enough to show the data.
/// Round means the scales will extend to show the data, and a little
/// more so that it ends at a nearby round number at the scale of 5 or 10.
/// Off means adjustment is not applied, and the minimum and maximum
/// values supplied by the user will be used instead.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Adjustment {
    Tight,
    Round,
    Off
}

impl FromStr for Adjustment {

    type Err = ();

    fn from_str(s : &str) -> Result<Self, ()> {
        match s {
            "tight" => Ok(Self::Tight),
            "round" => Ok(Self::Round),
            "off" => Ok(Self::Off),
            _ => Err(())
        }
    }

}

impl Default for Adjustment {

    fn default() -> Self {
        Self::Off
    }

}

#[derive(Debug, Clone)]
pub struct Scale {
    pub label : String,
    pub precision : i32,
    pub from : f64,
    pub to : f64,
    pub steps : Vec<f64>,
    pub n_intervals : i32,
    pub log : bool,
    pub invert : bool,
    pub offset : i32,
    pub adj : Adjustment
}

impl Default for Scale {

    fn default() -> Self {
        let mut s = Scale {
            label : String::new(),
            precision : 4,
            from : 0.0,
            to : 1.0,
            steps : vec![],
            n_intervals : 4,
            log : false,
            invert : false,
            offset : 0,
            adj : Adjustment::Tight
        };
        s.update_steps();
        s
    }
}

impl Scale {

    pub fn label(mut self, label : &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn precision(mut self, precision : i32) -> Self {
        self.precision = precision;
        self
    }

    pub fn extension(mut self, from : f64, to : f64) -> Self {
        self.from = from;
        self.to = to;
        self.update_steps();
        self
    }

    pub fn intervals(mut self, n : i32) -> Self {
        self.n_intervals = n;
        self.update_steps();
        self
    }

    pub fn log(mut self, log : bool) -> Self {
        self.log = log;
        self.update_steps();
        self
    }

    pub fn invert(mut self, invert : bool) -> Self {
        self.invert = invert;
        self.update_steps();
        self
    }

    pub fn offset(mut self, offset : i32) -> Self {
        self.offset = offset;
        self.update_steps();
        self
    }

    pub fn adjustment(mut self, adj : Adjustment) -> Self {
        self.adj = adj;
        self.update_steps();
        self
    }

    pub fn new() -> Self {
        Self::new_from_json(Default::default()).unwrap()
    }

    pub fn update(&mut self, prop : ScaleProperty) {
        let mut is_label = false;
        match prop {
            ScaleProperty::Label(label) => {
                self.label = label;
                is_label = true;
            },
            ScaleProperty::Min(min) => {
                self.from = min;
            },
            ScaleProperty::Max(max) => {
                self.to = max;
            },
            ScaleProperty::Log(log) => {
                self.log = log;
            }
            ScaleProperty::Invert(invert) => {
                self.invert = invert;
            }
            ScaleProperty::GridOffset(off) => {
                self.offset = off;
            }
            ScaleProperty::Precision(prec) => {
                self.precision = prec;
            }
            ScaleProperty::NIntervals(n_int) => {
                self.n_intervals = n_int;
            },
            ScaleProperty::Adjustment(adj) => {
                self.adj = adj;
            }
        }
        if !is_label {
            self.update_steps();
        }
    }

    fn update_steps(&mut self) {
        self.steps = define_steps(self.n_intervals, self.from, self.to, self.offset, self.log);
    }

    pub fn new_from_json(rep : super::json::Scale) -> Option<Self> {
        let adj : Adjustment = if let Some(adj) = rep.adjust {
            adj.parse().ok()?
        } else {
            Adjustment::Off
        };
        Some(Self::new_full(rep.label, rep.precision, rep.from, rep.to, rep.n_intervals, rep.log, rep.invert, rep.offset, adj))
    }

    pub fn new_full(
        label : String,
        precision : i32,
        from : f64,
        to : f64,
        n_intervals : i32,
        log : bool,
        invert : bool,
        offset : i32,
        adj : Adjustment
    ) -> Scale {
        let steps = define_steps(n_intervals, from, to, offset, log);
        Scale{ label, precision, from, to, steps, log, invert, offset, n_intervals, adj }
    }

    pub fn description(&self) -> HashMap<String, String> {
        let mut desc = HashMap::new();
        desc.insert("label".into(), self.label.clone());
        desc.insert("precision".into(), self.precision.to_string());
        desc.insert("from".into(), self.from.to_string());
        desc.insert("to".into(), self.to.to_string());
        desc.insert("n_intervals".into(), self.n_intervals.to_string());
        desc.insert("invert".into(), self.invert.to_string());
        desc.insert("log_scaling".into(), self.log.to_string());
        desc.insert("grid_offset".into(), self.offset.to_string());
        desc
    }

    /*pub fn update_steps(&mut self, from : f64, to : f64) {
        self.from = from;
        self.to = to;
        self.steps = define_steps(self.n_intervals, from, to, self.offset, self.log);
    }*/

}

pub fn adjust_segment(seg : &mut Scale, adj : Adjustment, data_min : f64, data_max : f64) {
    match adj {
        Adjustment::Tight => {
            println!("Tight adjustment applied");
            *seg = seg.clone().extension(data_min, data_max);
        },
        Adjustment::Round => {
            println!("Rounded adjustment applied");
            let (ideal_min, ideal_max) = context_mapper::round_to_most_extreme(data_min, data_max);
            let (curr_min, curr_max) = (seg.from, seg.to);
            let ampl = (data_max - data_min).abs();
            let large_pad_from = (data_min - curr_min).abs() / ampl > 0.25;
            let large_pad_to = (data_max - curr_max).abs() / ampl > 0.25;
            let should_change = data_min < curr_min || data_max > curr_max || large_pad_from || large_pad_to;
            if should_change {
                *seg = seg.clone().extension(ideal_min, ideal_max);
            }
        },
        Adjustment::Off => {

        }
    }
}

fn define_steps(n_intervals : i32, from : f64, to : f64, offset : i32, log : bool) -> Vec<f64> {
    let off_prop = match log {
        true => (10. as f64).powf(((to.log10() - from.log10()) / n_intervals as f64)*(offset as f64 / 100.)),
        false => ((to - from) / n_intervals as f64)*(offset as f64 / 100.0)
    };
    let from_offset = match log {
        true => from*off_prop,
        false => from + off_prop
    };
    let intv_size = match log {
        true => (to.log10() - from.log10() - 2.*(off_prop).log10()  ) / (n_intervals as f64),
        false => (to - from - 2.0*off_prop ) / (n_intervals as f64)
    };
    let mut steps = Vec::<f64>::new();
    for i in 0..n_intervals+1 {
        let step = if log {
            (10.0 as f64).powf(from_offset.log10() + (i as f64)*intv_size)
        } else {
            from_offset + (i as f64)*intv_size
        };
        steps.push(step);
    }
    steps
}
