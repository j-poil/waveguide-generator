use crate::geometry_types::{CartesianPoint, ProfilePoint};
use std::f64::consts::PI;

pub trait OblateSpheroidClothoidWG {
    // Common parameters
    fn k(&self) -> f64;
    fn r_init(&self) -> f64;
    fn alpha_init(&self) -> f64;

    fn term_length(&self) -> f64;
    fn term_end_radius(&self) -> f64;

    // Common calculations
    fn generalized_os_distance(&self, z: f64, tan_alpha: f64) -> f64 {
        let a = (self.k() * self.r_init()).powi(2);
        let b = 2.0 * self.k() * self.r_init() * z * self.alpha_init().tan();
        let c = (z * tan_alpha).powi(2);
        (a + b + c).sqrt() + self.r_init() * (1.0 - self.k())
    }

    fn morph_function(&self, _theta: f64, _l: f64) -> Option<f64> {
        None
    }

    // Angle calculation (to be implemented by variants)
    fn calculate_tan_alpha(&self, theta: f64, l: f64) -> f64 {
        if let Some(val) = self.morph_function(theta, l) {
            ((val - self.r_init() * (1.0 - self.k())).powi(2)
                - (self.k() * self.r_init()).powi(2)
                - 2.0 * self.k() * self.r_init() * l * self.alpha_init().tan())
            .sqrt()
                / l
        } else {
            panic!("Either alpha or morph function must be defined")
        }
    }

    /// Generate profile points along one angle
    fn generate_profile(&self, length: f64, theta: f64, step_length: f64) -> Vec<ProfilePoint> {
        // first, calculate the profile for the generalized OS until L
        let resolution = (length / step_length) as usize + 1; // +1 to include the last point
        let mut profile: Vec<ProfilePoint> = (0..resolution)
            .map(|i| {
                let tan_alpha = self.calculate_tan_alpha(theta, length);
                let z = (i as f64) * step_length;
                ProfilePoint {
                    z,
                    r: self.generalized_os_distance(z, tan_alpha),
                    theta,
                }
            })
            .collect();
        // then add the termination with clothoid/euler spiral
        self.add_termination(&mut profile, step_length);

        profile
    }

    /// Adds a termination section to the profile using a clothoid (Euler spiral)
    /// @param profile: The existing profile points to which the termination will be added
    fn add_termination(&self, profile: &mut Vec<ProfilePoint>, step_length: f64) {
        let initial_curvature_angle = ((profile[profile.len() - 1].r
            - profile[profile.len() - 2].r)
            / (profile[profile.len() - 1].z - profile[profile.len() - 2].z))
            .atan(); // atan(dy/dx) for the first point
        for i in 0..(self.term_length() / step_length).round() as i64 {
            // theta_n = s_n**2 / (2*Rc*sc) + theta_init
            // => zn+1 = zn + step_length*cos(theta_n)
            let ultimate_point = profile.last().unwrap();
            let curvature_angle = initial_curvature_angle
                + (i as f64 * step_length).powi(2)
                    / (2.0 * self.term_length() * self.term_end_radius());
            profile.push(ProfilePoint {
                z: ultimate_point.z + step_length * (curvature_angle).cos(),
                r: ultimate_point.r + step_length * (curvature_angle).sin(),
                theta: ultimate_point.theta,
            });
        }
    }

    /// Generate full 3D mesh
    fn generate_mesh(
        &self,
        length: f64,
        azimuth_steps: usize,
        axial_step_length: f64,
    ) -> Vec<[CartesianPoint; 3]> {
        let theta_positions: Vec<f64> = (0..azimuth_steps)
            .map(|i| 2.0 * PI * (i as f64) / (azimuth_steps as f64))
            .collect();

        let profiles: Vec<Vec<ProfilePoint>> = theta_positions
            .iter()
            .map(|&theta| self.generate_profile(length, theta, axial_step_length))
            .collect();

        let mut triangles = Vec::new();

        for profile_idx in 0..azimuth_steps {
            let next_profile_idx = (profile_idx + 1) % azimuth_steps;

            let current_profile = &profiles[profile_idx];
            let next_profile = &profiles[next_profile_idx];

            let nb_axial_steps = current_profile.len();
            println!("nb_axial_steps : {}", nb_axial_steps);

            for point_idx in 0..(nb_axial_steps - 1) {
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
