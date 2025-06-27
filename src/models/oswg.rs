use super::base::Waveguide;
use crate::geometry_types::{CartesianPoint, ProfilePoint};
use std::f64::consts::PI;

pub trait OblateSpheroidWG: Waveguide {
    // Common parameters
    fn k(&self) -> f64;
    fn r_init(&self) -> f64;
    fn alpha_init(&self) -> f64;
    fn s(&self) -> f64;
    fn q(&self) -> f64;
    fn n(&self) -> f64;

    // Common calculations
    fn generalized_os_distance(&self, z: f64, tan_alpha: f64) -> f64 {
        let a = (self.k() * self.r_init()).powi(2);
        let b = 2.0 * self.k() * self.r_init() * z * self.alpha_init().tan();
        let c = (z * tan_alpha).powi(2);
        (a + b + c).sqrt() + self.r_init() * (1.0 - self.k())
    }

    fn termination_distance(&self, z: f64, l: f64) -> f64 {
        self.s() * l / self.q()
            * (1.0 - (1.0 - (z * self.q() / l).powf(self.n())).powf(1.0 / self.n()))
    }

    fn morph_function(&self, _theta: f64, _l: f64) -> Option<f64> {
        None
    }

    // Angle calculation (to be implemented by variants)
    fn calculate_tan_alpha(&self, theta: f64, l: f64) -> f64 {
        if let Some(val) = self.morph_function(theta, l) {
            ((val - self.termination_distance(l, l) - self.r_init() * (1.0 - self.k())).powi(2)
                - (self.k() * self.r_init()).powi(2)
                - 2.0 * self.k() * self.r_init() * l * self.alpha_init().tan())
            .sqrt()
                / l
        } else {
            panic!("Either alpha or morph function must be defined")
        }
    }

    fn radial_distance(&self, z: f64, theta: f64, l: f64) -> f64 {
        let tan_alpha = self.calculate_tan_alpha(theta, l);
        self.generalized_os_distance(z, tan_alpha) + self.termination_distance(z, l)
    }

    /// Generate profile points along one angle
    fn generate_profile(&self, length: f64, theta: f64, resolution: usize) -> Vec<ProfilePoint> {
        (0..resolution)
            .map(|i| {
                let z = length * (i as f64) / ((resolution - 1) as f64);
                ProfilePoint {
                    z,
                    r: self.radial_distance(z, theta, length),
                    theta,
                }
            })
            .collect()
    }

    /// Generate full 3D mesh
    fn generate_mesh(
        &self,
        length: f64,
        azimuth_steps: usize,
        axial_steps: usize,
    ) -> Vec<[CartesianPoint; 3]> {
        let theta_positions: Vec<f64> = (0..azimuth_steps)
            .map(|i| 2.0 * PI * (i as f64) / (azimuth_steps as f64))
            .collect();

        let profiles: Vec<Vec<ProfilePoint>> = theta_positions
            .iter()
            .map(|&theta| self.generate_profile(length, theta, axial_steps))
            .collect();

        let mut triangles = Vec::new();

        for profile_idx in 0..azimuth_steps {
            let next_profile_idx = (profile_idx + 1) % azimuth_steps;

            let current_profile = &profiles[profile_idx];
            let next_profile = &profiles[next_profile_idx];

            for point_idx in 0..(axial_steps - 1) {
                let p0 = current_profile[point_idx];
                let p1 = current_profile[point_idx + 1];
                let p2 = next_profile[point_idx];
                let p3 = next_profile[point_idx + 1];

                // Triangle 1 (p0, p2, p1) - CCW for outward normals
                triangles.push([
                    CartesianPoint::from_cylindrical(p0.r, p0.theta, p0.z),
                    CartesianPoint::from_cylindrical(p2.r, p2.theta, p2.z),
                    CartesianPoint::from_cylindrical(p1.r, p1.theta, p1.z),
                ]);

                // Triangle 2 (p1, p2, p3)
                triangles.push([
                    CartesianPoint::from_cylindrical(p1.r, p1.theta, p1.z),
                    CartesianPoint::from_cylindrical(p2.r, p2.theta, p2.z),
                    CartesianPoint::from_cylindrical(p3.r, p3.theta, p3.z),
                ]);
            }
        }

        triangles
    }
}

impl<T: OblateSpheroidWG> Waveguide for T {
    fn generate_mesh(
        &self,
        length: f64,
        azimuth_steps: usize,
        axial_steps: usize,
    ) -> Vec<[CartesianPoint; 3]> {
        OblateSpheroidWG::generate_mesh(self, length, azimuth_steps, axial_steps)
    }
}
