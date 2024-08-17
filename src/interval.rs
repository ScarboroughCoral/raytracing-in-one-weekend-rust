pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Default for Interval {
    fn default() -> Self {
        // Default interval is empty
        Interval::EMPTY
    }
}

impl Interval {
    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: -f64::INFINITY };
    pub const UNIVERSE: Interval = Interval { min: -f64::INFINITY, max: f64::INFINITY  };
}


