use crate::triangle::Triangle;
use crate::vector3::Vector3;
use std::fs::File;
use std::io::ErrorKind::InvalidData;
use std::io::{BufRead, BufReader, Error};

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
        self.triangles.push(triangle)
    }

    pub fn load_stl_ascii(file_path: &str) -> Result<Self, Error> {
        let file = File::open(file_path);
        let reader = BufReader::new(file?);

        let mut mesh = Self::new();
        let mut vertices: Vec<Vector3> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "vertex" if parts.len() == 4 => {
                    let vertex = Vector3::new(
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                        parts[3].parse().unwrap(),
                    );
                    vertices.push(vertex)
                }
                "endfacet" => {
                    if vertices.len() == 3 {
                        let triangle = Triangle::new(vertices[0], vertices[1], vertices[2]);
                        mesh.push(triangle)
                    } else {
                        return Err(Error::new(InvalidData, "Couldn't parse triangles."));
                    }
                    vertices = Vec::new();
                }
                _ => {}
            }
        }

        Ok(mesh)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::Mesh;
    use crate::triangle::Triangle;
    use crate::vector3::Vector3;

    #[test]
    fn can_push_triangle_to_mesh() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let c = Vector3::new(1.0, 1.0, 0.0);
        let triangle = Triangle::new(a, b, c);

        let mut mesh = Mesh::new();
        mesh.push(triangle);

        assert_eq!(mesh.triangles.len(), 1);
    }

    #[test]
    fn can_load_stl_ascii() {
        let file_path = "test_data/cube.stl";
        let mesh = Mesh::load_stl_ascii(file_path).unwrap();
        assert_eq!(mesh.triangles.len(), 12);
        for triangle in &mesh.triangles {
            eprintln!("{}", triangle);
        }
    }
}
