use crate::models::OblateSpheroidClothoidWG;

pub struct RectOSCWG {
    pub k: f64,
    pub r_init: f64,
    pub alpha_init: f64,
    pub term_end_radius: f64,
    pub term_length: f64,
    pub alpha_h: f64,
    pub alpha_v: f64,
}

impl OblateSpheroidClothoidWG for RectOSCWG {
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

    fn calculate_tan_alpha(&self, theta: f64, _l:f64) -> f64 {
        let h_axis = self.alpha_h.tan();
        let v_axis = self.alpha_v.tan();

        (h_axis/theta.cos().abs()).min(v_axis/theta.sin().abs()) // simplified l in tan(alpha) and tan(h_axis)
    }
}
