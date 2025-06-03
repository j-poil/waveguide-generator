
/// 3D Point in Cartesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct CartesianPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CartesianPoint {
    /// Convert from cylindrical coordinates (r, θ, z)
    /// where θ is in radians
    pub fn from_cylindrical(r: f64, theta: f64, z: f64) -> Self {
        Self {
            x: r * theta.cos(),
            y: r * theta.sin(),
            z,
        }
    }
}

/// Profile point in cylindrical coordinates
/// Represents a single point along the waveguide's generatrix
#[derive(Debug, Clone, Copy)]
pub struct ProfilePoint {
    pub z: f64,     // axial position
    pub r: f64,     // radial distance from axis
    pub theta: f64, // azimuthal angle (constant for a single profile)
}
