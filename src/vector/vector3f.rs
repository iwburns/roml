use super::Vector;
use super::Vector3;

pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector<f32> for Vector3f {
    fn add(&mut self, v: &Vector3f) -> &mut Vector3f {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }

    fn add_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        dest.x = self.x + v.x;
        dest.y = self.y + v.y;
        dest.z = self.z + v.z;
    }

    fn angle(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn angle_cos(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn distance(&self, v: &Vector3f) -> f32 {
        self.distance_squared(v).sqrt()
    }

    fn distance_squared(&self, v: &Vector3f) -> f32 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        (dx * dx) + (dy * dy) + (dz * dz)
    }

    fn dot(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn fma_scalar(&mut self, a: f32, b: &Vector3f) -> &mut Vector3f {
        unimplemented!()
    }

    fn fma_scalar_into(&self, a: f32, b: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn fma_vector(&mut self, a: &Vector3f, b: &Vector3f) -> &mut Vector3f {
        unimplemented!()
    }

    fn fma_vector_into(&self, a: &Vector3f, b: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn length(&self) -> f32 {
        unimplemented!()
    }

    fn length_squared(&self) -> f32 {
        unimplemented!()
    }

    fn lerp(&mut self, other: &Vector3f, t: f32) -> &mut Vector3f {
        unimplemented!()
    }

    fn lerp_into(&self, other: &Vector3f, t: f32, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn mul_scalar(&mut self, s: f32) -> &mut Vector3f {
        unimplemented!()
    }

    fn mul_scalar_into(&self, s: f32, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn mul_vector(&mut self, v: &Vector3f) -> &mut Vector3f {
        unimplemented!()
    }

    fn mul_vector_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn negate(&mut self) -> &mut Vector3f {
        unimplemented!()
    }

    fn negate_into(&self, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn normalize(&mut self) -> &mut Vector3f {
        unimplemented!()
    }

    fn normalize_into(&self, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn set(&mut self, v: &Vector3f) -> &mut Vector3f {
        unimplemented!()
    }

    fn sub(&mut self, v: &Vector3f) -> &mut Vector3f {
        unimplemented!()
    }

    fn sub_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn zero(&mut self) -> &mut Vector3f {
        unimplemented!()
    }
}

impl Vector3<f32> for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f {
            x: x,
            y: y,
            z: z,
        }
    }

    fn add_components(&mut self, x: f32, y: f32, z: f32) -> &mut Vector3f {
        unimplemented!()
    }

    fn add_components_into(&self, x: f32, y: f32, z: f32, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn cross(&mut self, v: &Vector3f) -> &mut Vector3f {
        unimplemented!()
    }

    fn cross_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Vector;
    use super::super::Vector3;
    use super::Vector3f;

    use std;

    #[test]
    fn test_new() {
        let a = Vector3f::new(1f32, 2f32, 3f32);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
        assert_eq!(a.z, 3f32);
    }

    #[test]
    fn test_add() {
        let mut a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(1f32, 2f32, 3f32);

        a.add(&b);

        assert_eq!(a.x, 2f32);
        assert_eq!(a.y, 4f32);
        assert_eq!(a.z, 6f32);
    }

    #[test]
    fn test_add_into() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(1f32, 2f32, 3f32);
        let mut c = Vector3f::new(0f32, 0f32, 0f32);

        a.add_into(&b, &mut c);

        assert_eq!(c.x, 2f32);
        assert_eq!(c.y, 4f32);
        assert_eq!(c.z, 6f32);
    }

    #[test]
    fn test_distance() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(-1f32, 2f32, 3f32);

        let target_distance = 2f32;
        let distance = a.distance(&b);

        assert_eq!(target_distance, distance);
    }

    #[test]
    fn test_distance_squared() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(-1f32, 2f32, 3f32);

        let target_distance_sq = 4f32; // target distance is 2
        let distance_sq = a.distance_squared(&b);

        assert_eq!(target_distance_sq, distance_sq);
    }
}
