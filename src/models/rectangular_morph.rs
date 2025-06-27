use crate::models::OblateSpheroidWG;

pub struct RectangularMorphOSWG {
    pub k: f64,
    pub r_init: f64,
    pub alpha_init: f64,
    pub s: f64,
    pub q: f64,
    pub n: f64,
    pub alpha_h: f64,
    pub alpha_v: f64,
}

impl OblateSpheroidWG for RectangularMorphOSWG {
    fn k(&self) -> f64 { self.k }
    fn r_init(&self) -> f64 { self.r_init }
    fn alpha_init(&self) -> f64 { self.alpha_init }
    fn s(&self) -> f64 { self.s }
    fn q(&self) -> f64 { self.q }
    fn n(&self) -> f64 { self.n }

    fn morph_function(&self, theta: f64, l:f64) -> Option<f64> {
        let h_axis = self.alpha_h.tan()*l;
        let v_axis = self.alpha_v.tan()*l;

        Some((h_axis/theta.cos().abs()).min(v_axis/theta.sin().abs())) // simplified l in tan(alpha) and tan(h_axis)
    }
}