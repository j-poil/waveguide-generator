use crate::models::OblateSpheroidWG;

pub struct EllipsoidalOSWG {
    pub k: f64,
    pub r_init: f64,
    pub alpha_init: f64,
    pub s: f64,
    pub q: f64,
    pub n: f64,
    pub alpha_h: f64,
    pub alpha_v: f64,
}

impl OblateSpheroidWG for EllipsoidalOSWG {
    fn k(&self) -> f64 { self.k }
    fn r_init(&self) -> f64 { self.r_init }
    fn alpha_init(&self) -> f64 { self.alpha_init }
    fn s(&self) -> f64 { self.s }
    fn q(&self) -> f64 { self.q }
    fn n(&self) -> f64 { self.n }

    fn calculate_alpha(&self, theta: f64) -> f64 {
        let h_axis = self.alpha_h.tan();
        let v_axis = self.alpha_v.tan();
        let r = (h_axis * v_axis) /
            ((h_axis * theta.cos()).powi(2) + (v_axis * theta.sin()).powi(2)).sqrt();
        r.atan()
    }
}