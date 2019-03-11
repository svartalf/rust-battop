use itertools::{Itertools, MinMaxResult};

const RESOLUTION: usize = 512;

#[derive(Debug)]
pub enum GraphUnits {
    Volt,
    Watt,
    Celsius,
}

#[derive(Debug)]
pub struct GraphData<'g> {
    title: &'g str,
    units: GraphUnits,
    points: Vec<(f64, f64)>,

    value_latest: f64,
    value_min: f64,
    value_max: f64,
}

impl<'g> GraphData<'g> {
    pub fn new(title: &'g str, units: GraphUnits) -> GraphData {
        GraphData {
            title,
            units,
            points: Vec::with_capacity(RESOLUTION),
            value_latest: 0.0,
            value_min: 100.0,
            value_max: 0.0,
        }
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn current(&self) -> String {
        match self.units {
            GraphUnits::Volt => format!("{:.2} V", self.value_latest),
            GraphUnits::Watt => format!("{:.2} W", self.value_latest),
            GraphUnits::Celsius => format!("{:.2} °C", self.value_latest),
        }
    }

    pub fn points(&self) -> &[(f64, f64)] {
        self.points.as_ref()
    }

    // X - time

    pub fn x_bounds(&self) -> [f64; 2] {
        [0.0, 256.0]
    }

    // Y - values

    pub fn y_title(&self) -> &str {
        match self.units {
            GraphUnits::Volt => "V",
            GraphUnits::Watt => "W",
            GraphUnits::Celsius => "°C",
        }
    }

    fn y_lower(&self) -> f64 {
        let mut value = (self.value_min - 1.0).floor();
        if value < 0.0 {
            value = -1.0;
        }
        value
    }

    fn y_upper(&self) -> f64 {
        (self.value_max + 1.0).ceil()
    }

    pub fn y_labels(&self) -> Vec<String> {
        vec![
            format!("{:2.0}", self.y_lower()),
            format!("{:2.0}", self.y_upper()),
        ]
    }

    pub fn y_bounds(&self) -> [f64; 2] {
        [self.y_lower(), self.y_upper()]
    }

    #[allow(clippy::cast_lossless)]
    pub fn push(&mut self, value: f64) {
        if self.points.len() == RESOLUTION {
            self.points.remove(0);
        }
        for (x, _) in self.points.iter_mut() {
            *x -= 0.5;
        }

        self.value_latest = value;

        self.points.push((RESOLUTION as f64 / 2.0, value));
        match self.points.iter().minmax_by_key(|(_, y)| y) {
            MinMaxResult::MinMax((_, min), (_, max)) => {
                self.value_min = *min;
                self.value_max = *max;
            },
            MinMaxResult::OneElement((_, el)) => {
                self.value_min = *el;
                self.value_max = *el;
            }
            _ => {}
        }
    }
}

