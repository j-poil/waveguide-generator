use crate::geometry_types::{CartesianPoint};

pub trait Waveguide {
    // fn length(&self) -> f64; // length can be variable (maybe should be function of theta ?)
    fn generate_mesh(&self, length: f64, azimuth_steps: usize, axial_steps: usize) -> Vec<[CartesianPoint; 3]>;
}
