mod geometry_types;
mod models;

use geometry_types::{CartesianPoint, ProfilePoint};

use crate::models::{EllipsoidalOSWG, OblateSpheroidWG, OblateSpheroidClothoidWG, AxisymOSWG, RectangularOSWG, RectangularMorphOSWG};
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use stl_io::{Normal, Triangle, Vertex};

/// Writes profile points to CSV file (for debugging/visualization)
fn export_coordinates_to_csv(points: &[ProfilePoint], filename: &str) -> std::io::Result<()> {
    #[derive(Serialize)]
    struct CsvPoint {
        z: f64,
        r: f64,
        theta: f64,
        x: f64,
        y: f64,
    }

    let mut writer = csv::Writer::from_path(filename)?;

    for point in points {
        let cartesian = CartesianPoint::from_cylindrical(point.r, point.theta, point.z);
        writer.serialize(CsvPoint {
            z: point.z,
            r: point.r,
            theta: point.theta,
            x: cartesian.x,
            y: cartesian.y,
        })?;
    }

    Ok(())
}

/// Calculates normal vector for a triangle (points in CCW order)
fn triangle_normal(v0: &CartesianPoint, v1: &CartesianPoint, v2: &CartesianPoint) -> Normal {
    let u = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
    let v = [v2.x - v0.x, v2.y - v0.y, v2.z - v0.z];

    // Cross product u × v
    Normal::new([
        (u[1] * v[2] - u[2] * v[1]) as f32,
        (u[2] * v[0] - u[0] * v[2]) as f32,
        (u[0] * v[1] - u[1] * v[0]) as f32,
    ])
}

/// Exports waveguide mesh to an STL file
fn export_stl(mesh: &[[CartesianPoint; 3]], filename: &str) -> std::io::Result<()> {
    let mut mesh_triangles: Vec<Triangle> = Vec::new();
    for triangle in mesh {
        // Calculate normal using original CartesianPoints for better precision
        let normal = triangle_normal(&triangle[0], &triangle[1], &triangle[2]);

        // Create an indexed triangle
        mesh_triangles.push(Triangle {
            normal,
            vertices: [
                Vertex::new([
                    triangle[0].x as f32,
                    triangle[0].y as f32,
                    triangle[0].z as f32,
                ]),
                Vertex::new([
                    triangle[1].x as f32,
                    triangle[1].y as f32,
                    triangle[1].z as f32,
                ]),
                Vertex::new([
                    triangle[2].x as f32,
                    triangle[2].y as f32,
                    triangle[2].z as f32,
                ]),
            ],
        });
    }

    // Write to a file
    let mut file = BufWriter::new(File::create(filename)?);
    stl_io::write_stl(&mut file, mesh_triangles.iter())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let waveguide_length = 200.0; // mm
    let azimuthal_steps = 36; // 10° resolution
    let axial_steps = 50;

    let ellipsoidal = EllipsoidalOSWG {
        k: 1.0,
        r_init: 25.4,
        alpha_init: 1.0f64.to_radians(),
        s: 0.7,
        q: 0.997,
        n: 6.0,
        alpha_h: 45.0f64.to_radians(),
        alpha_v: 30.0f64.to_radians(),
    };
    // Generate and export a sample profile for inspection
    let test_profile = ellipsoidal.generate_profile(waveguide_length, 0.0, axial_steps);
    export_coordinates_to_csv(&test_profile, "target/exports/waveguide_profile.csv")?;

    // Generate full 3D mesh and export
    let triangles = ellipsoidal.generate_mesh(waveguide_length, azimuthal_steps, axial_steps);
    export_stl(&triangles, "target/exports/ellipsoidal.stl")?;

    let axisym = AxisymOSWG {
        k: 1.0,
        r_init: 25.4,
        alpha_init: 1.0f64.to_radians(),
        s: 0.7,
        q: 0.997,
        n: 6.0,
        alpha: 45.0f64.to_radians(),
    };
    let axi_triangles = axisym.generate_mesh(waveguide_length, azimuthal_steps, axial_steps);
    export_stl(&axi_triangles, "target/exports/axisymmetric.stl")?;

    let rectangular =  RectangularOSWG {
        k: 1.0,
        r_init: 25.4,
        alpha_init: 1.0f64.to_radians(),
        s: 0.7,
        q: 0.997,
        n: 6.0,
        alpha_h: 45.0f64.to_radians(),
        alpha_v: 30.0f64.to_radians(),
    };
    let rect_triangles = rectangular.generate_mesh(waveguide_length, azimuthal_steps, axial_steps);
    export_stl(&rect_triangles, "target/exports/rectangular_alpha.stl")?;
    
    let rectangular_morph =  RectangularMorphOSWG {
        k: 1.0,
        r_init: 25.4,
        alpha_init: 1.0f64.to_radians(),
        s: 0.7,
        q: 0.997,
        n: 6.0,
        alpha_h: 45.0f64.to_radians(),
        alpha_v: 30.0f64.to_radians(),
    };
    let rect_morph_triangles = rectangular_morph.generate_mesh(waveguide_length, azimuthal_steps, axial_steps);
    export_stl(&rect_morph_triangles, "target/exports/rectangular_morph.stl")?;

    let axisym_clothoid = models::AxisymOSCWG {
        k: 1.0,
        r_init: 25.4,
        alpha_init: 1.0f64.to_radians(),
        term_length: 200.0, // mm
        term_end_radius: 60.0, // mm
        alpha: 45.0f64.to_radians(),
    };
    let test_profile = axisym_clothoid.generate_profile(waveguide_length, 0.0, 4.0);
    export_coordinates_to_csv(&test_profile, "target/exports/clothoid_waveguide_profile.csv")?;
    let axi_clothoid_triangles = axisym_clothoid.generate_mesh(waveguide_length, 2*azimuthal_steps, 4.0);
    export_stl(&axi_clothoid_triangles, "target/exports/axi_clothoid_triangles.stl")?;

    let rect_clothoid = models::RectOSCWG {
        k: 1.0,
        r_init: 25.4,
        alpha_init: 1.0f64.to_radians(),
        term_length: 180.0, // mm
        term_end_radius: 50.0, // mm
        alpha_h: 45.0f64.to_radians(),
        alpha_v: 30.0f64.to_radians(),
    };

    let rect_clothoid_triangles = rect_clothoid.generate_mesh(waveguide_length, azimuthal_steps, 4.0);
    export_stl(&rect_clothoid_triangles, "target/exports/rect_clothoid.stl")?;

    println!("Successfully exported waveguide data");
    Ok(())
}
