use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;

fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}


fn multiply_matrix_vector3(matrix: &Matrix, vector: &Vector3) -> Vector3 {
    Vector3::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z,
    )
}

fn get_base_color_from_normal(normal: &Vector3) -> Vector3 {
    // Normalize the normal vector
    let len = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let n = if len > 0.0001 {
        Vector3::new(normal.x / len, normal.y / len, normal.z / len)
    } else {
        Vector3::new(0.0, 1.0, 0.0)
    };


    let abs_x = n.x.abs();
    let abs_y = n.y.abs();
    let abs_z = n.z.abs();

    if abs_y > abs_x && abs_y > abs_z {

        Vector3::new(0.0, 1.0, 1.0)
    } else if abs_x > abs_y && abs_x > abs_z {

        Vector3::new(0.0, 0.0, 0.0)
    } else {

        Vector3::new(0.5, 0.5, 0.5)
    }
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {

    let position_vec4 = Vector4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );


    let transformed_vec4 = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);


    let transformed_position = if transformed_vec4.w != 0.0 {
        Vector3::new(
            transformed_vec4.x / transformed_vec4.w,
            transformed_vec4.y / transformed_vec4.w,
            transformed_vec4.z / transformed_vec4.w,
        )
    } else {
        Vector3::new(transformed_vec4.x, transformed_vec4.y, transformed_vec4.z)
    };


    let transformed_normal = multiply_matrix_vector3(&uniforms.model_matrix, &vertex.normal);


    let color = get_base_color_from_normal(&transformed_normal);


    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color,
        transformed_position,
        transformed_normal,
    }
}