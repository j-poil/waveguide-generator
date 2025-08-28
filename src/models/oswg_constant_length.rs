use crate::geometry_types::{CartesianPoint, ProfilePoint};
use std::f64::consts::PI;

pub trait ConstantLengthOblateSpheroidWG: OblateSpheroidWG {
    // Common parameters
    fn k(&self) -> f64;
    fn r_init(&self) -> f64;
    fn alpha_init(&self) -> f64;
    fn s(&self) -> f64;
    fn q(&self) -> f64;
    fn n(&self) -> f64;
    fn l(&self) -> f64;
    fn curve_length(&self) -> f64; // can't be a method (or would be calculated rach time)

    fn calculate_profile_curve_length(&self, profile: &Vec<ProfilePoint>) -> f64 {
        // Calculate the length of the profile curve using the trapezoidal rule
        profile
            .windows(2)
            .map(|pair| {
                let dx = pair[1].z - pair[0].z;
                let avg_r = (pair[0].r + pair[1].r) / 2.0;
                (dx * avg_r).hypot()
            })
            .sum()
    }
    /// Generat e profile points along one angle
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

    fn generate_profile_with_fixed_length(
        &self,
        theta: f64,
        resolution: usize,
    ) -> Vec<ProfilePoint> {
        let mut max_lenth = self.l() * 2.0;
        let mut min_lenth = self.l() / 2.0;
        let mut profile = self.generate_profile(length, theta, resolution);

        // Adjust the profile to ensure it has the correct length
        let mut current_length = self.calculate_profile_curve_length(&profile);
        while (current_length - self.curve_length()).abs() > 1e-6 {
            if current_length < self.curve_length() {
                min_lenth = current_length
            } else {
                max_lenth = current_length;
            }
            current_length = (max_lenth - min_lenth) / 2.0 + min_lenth;
            profile = self.generate_profile(current_length, theta, resolution);

        }

        profile
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
