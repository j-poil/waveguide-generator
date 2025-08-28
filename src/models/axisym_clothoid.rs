use crate::models::OblateSpheroidClothoidWG;

pub struct AxisymOSCWG {
    pub k: f64,
    pub r_init: f64,
    pub alpha_init: f64,
    pub term_end_radius: f64,
    pub term_length: f64,
    pub alpha: f64,
}

impl OblateSpheroidClothoidWG for AxisymOSCWG {
    fn k(&self) -> f64 {
        self.k
    }
    fn r_init(&self) -> f64 {
        self.r_init
    }
    fn alpha_init(&self) -> f64 {
        self.alpha_init
    }
    fn term_length(&self) -> f64 {
        self.term_length
    }
    fn term_end_radius(&self) -> f64 {
        self.term_end_radius
    }

    fn calculate_tan_alpha(&self, _theta: f64, _l: f64) -> f64 {
        self.alpha.tan()
    }
}
