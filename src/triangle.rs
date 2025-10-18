use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::math::Vector3;

/// Calculate the area of a triangle using the cross product
fn triangle_area(p1: &Vector3, p2: &Vector3, p3: &Vector3) -> f32 {
    let v1 = Vector3::new(p2.x - p1.x, p2.y - p1.y, 0.0);
    let v2 = Vector3::new(p3.x - p1.x, p3.y - p1.y, 0.0);
    (v1.x * v2.y - v1.y * v2.x).abs() / 2.0
}

/// Calculate barycentric coordinates for a point within a triangle
fn barycentric_coordinates(
    p: &Vector3,
    v1: &Vector3,
    v2: &Vector3,
    v3: &Vector3,
) -> Option<(f32, f32, f32)> {
    let area = triangle_area(v1, v2, v3);
    if area < 0.0001 {
        return None; // Degenerate triangle
    }

    let w1 = triangle_area(p, v2, v3) / area;
    let w2 = triangle_area(v1, p, v3) / area;
    let w3 = triangle_area(v1, v2, p) / area;

    Some((w1, w2, w3))
}

/// Interpolate a value across the triangle using barycentric coordinates
fn interpolate(v1: f32, v2: f32, v3: f32, w1: f32, w2: f32, w3: f32) -> f32 {
    v1 * w1 + v2 * w2 + v3 * w3
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;
    let p3 = v3.transformed_position;

    // Get bounding box
    let min_x = p1.x.min(p2.x).min(p3.x).floor() as i32;
    let max_x = p1.x.max(p2.x).max(p3.x).ceil() as i32;
    let min_y = p1.y.min(p2.y).min(p3.y).floor() as i32;
    let max_y = p1.y.max(p2.y).max(p3.y).ceil() as i32;

    // Calculate the area of the triangle to determine winding order
    let area = (p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y);
    

    if area < 0.0 {
        return fragments;
    }

    // Iterate over all pixels in the bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel = Vector3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

            // Check if pixel is inside the triangle using barycentric coordinates
            if let Some((w1, w2, w3)) = barycentric_coordinates(&pixel, &p1, &p2, &p3) {
                // All weights must be >= 0 for the point to be inside
                if w1 >= -0.0001 && w2 >= -0.0001 && w3 >= -0.0001 {
                    // Interpolate depth
                    let z = interpolate(p1.z, p2.z, p3.z, w1, w2, w3);

                    // Interpolate color
                    let color = Vector3::new(
                        interpolate(v1.color.x, v2.color.x, v3.color.x, w1, w2, w3),
                        interpolate(v1.color.y, v2.color.y, v3.color.y, w1, w2, w3),
                        interpolate(v1.color.z, v2.color.z, v3.color.z, w1, w2, w3),
                    );

                    fragments.push(Fragment::new(x as f32, y as f32, color, z));
                }
            }
        }
    }

    fragments
}