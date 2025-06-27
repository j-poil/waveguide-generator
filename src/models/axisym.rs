use crate::models::OblateSpheroidWG;

pub struct AxisymOSWG {
    pub k: f64,
    pub r_init: f64,
    pub alpha_init: f64,
    pub s: f64,
    pub q: f64,
    pub n: f64,
    pub alpha: f64,
}

impl OblateSpheroidWG for AxisymOSWG {
    fn k(&self) -> f64 { self.k }
    fn r_init(&self) -> f64 { self.r_init }
    fn alpha_init(&self) -> f64 { self.alpha_init }
    fn s(&self) -> f64 { self.s }
    fn q(&self) -> f64 { self.q }
    fn n(&self) -> f64 { self.n }

    fn calculate_tan_alpha(&self, _theta: f64, _l:f64) -> f64 {
        self.alpha.tan()
    }
}