use crate::linalg::vector3::Vector3;
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Triangle {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl Triangle {
    pub fn new(a: Vector3, b: Vector3, c: Vector3) -> Self {
        Self { a, b, c }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Triangle<{}, {}, {}>", self.a, self.b, self.c)
    }
}

#[cfg(test)]
mod tests {
    use crate::linalg::vector3::Vector3;
    use crate::triangle::Triangle;

    #[test]
    fn can_create_triangle() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let c = Vector3::new(1.0, 1.0, 0.0);
        let triangle = Triangle::new(a, b, c);

        assert_eq!(triangle.a, a);
        assert_eq!(triangle.b, b);
        assert_eq!(triangle.c, c);
    }
}
