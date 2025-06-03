use crate::traits::WaveguideModel;

pub struct OblateSpheroidWG {
    pub k: f64,
    pub r_init: f64,
    pub alpha_init: f64,
    pub s: f64,
    pub q: f64,
    pub n: f64,
    pub alpha_h: f64,
    pub alpha_v: f64,
}

impl OblateSpheroidWG {
    pub fn new() -> Self {
        Self {
            k: 1.0,
            r_init: 254.0,
            alpha_init: 1.0f64.to_radians(),
            s: 0.7,
            q: 0.997,
            n: 6.0,
            alpha_h: 45.0f64.to_radians(),
            alpha_v: 30.0f64.to_radians(),
        }
    }
}

impl WaveguideModel for OblateSpheroidWG {
    fn radial_distance(&self, z: f64, theta: f64, l:f64) -> f64 {
        let alpha = self.calculate_alpha(theta);
        self.generalized_os_distance(z, alpha) + self.termination_distance(z, l)
    }
}

impl OblateSpheroidWG {
    fn calculate_alpha(&self, theta: f64) -> f64 {
        let h_axis = self.alpha_h.tan();
        let v_axis = self.alpha_v.tan();
        let r = (h_axis * v_axis) / (h_axis.powi(2) * theta.cos().powi(2) + v_axis.powi(2) * theta.sin().powi(2)).sqrt();
        r.atan()
    }

    fn generalized_os_distance(&self, z: f64, alpha: f64) -> f64 {
        let a = (self.k * self.r_init).powi(2);
        let b = 2.0 * self.k * self.r_init * z * self.alpha_init.tan();
        let c = (z * alpha.tan()).powi(2);
        (a + b + c).sqrt() + self.r_init * (1.0 - self.k)
    }

    fn termination_distance(&self, z: f64, l:f64) -> f64 {
        self.s * l / self.q * (1.0 - (1.0 - (z * self.q / l).powf(self.n)).powf(1.0/self.n))
    }
}