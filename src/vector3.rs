use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self::new(self.x / len, self.y / len, self.z / len)
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector3::Vector3;

    #[test]
    fn can_create_vector() {
        let vector3 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(vector3.x, 1.0);
        assert_eq!(vector3.y, 2.0);
        assert_eq!(vector3.z, 3.0);
    }

    #[test]
    fn can_calculate_dot_product() {
        let first = Vector3::new(1.0, 2.0, 3.0);
        let second = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(first.dot(&second), 32f64);
    }
    #[test]
    fn can_calculate_length() {
        let vector3 = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(vector3.length(), 1f64);
    }
    #[test]
    fn can_normalize() {
        let vector3 = Vector3::new(10.0, 0.0, 0.0);
        let normalized = vector3.normalize();
        assert_eq!(normalized.x, 1f64);
        assert_eq!(normalized.y, 0f64);
        assert_eq!(normalized.z, 0f64);
    }
}
