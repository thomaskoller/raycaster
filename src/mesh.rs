use crate::triangle::Triangle;

#[derive(PartialEq, Clone, Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    pub fn push(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }
}

#[cfg(test)]
mod tests {}
#[test]
fn can_push_triangle_to_mesh() {
    use crate::vector3::Vector3;

    let a = Vector3::new(1.0, 0.0, 0.0);
    let b = Vector3::new(0.0, 1.0, 0.0);
    let c = Vector3::new(1.0, 1.0, 0.0);
    let triangle = Triangle::new(a, b, c);

    let mut mesh = Mesh::new();
    mesh.push(triangle);

    assert_eq!(mesh.triangles.len(), 1);
}
