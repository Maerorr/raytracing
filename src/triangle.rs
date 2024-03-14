use crate::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Vector; 3],
    pub normal: Vector,
    pub color: Vector,
}

impl Triangle {
    pub fn new(vertices: [Vector; 3], color: Vector) -> Triangle {

        let mut normal = (vertices[1] - vertices[0]).cross(&(vertices[2] - vertices[0]));
        normal.normalize();
        Triangle { vertices, normal, color }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Triangle: vertices: {:?}, normal: {:?}, color: {:?}",
            self.vertices, self.normal, self.color
        )
    }
}

