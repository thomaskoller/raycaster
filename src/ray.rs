use crate::vector3::Vector3;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}
#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vector3::Vector3;

    #[test]
    fn can_create_ray() {
        let origin = Vector3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(1.0, 2.0, 3.0);
        let result = Ray::new(origin.clone(), direction.clone());

        assert_eq!(result.origin, origin);
        assert_eq!(result.direction, direction);
    }
}
