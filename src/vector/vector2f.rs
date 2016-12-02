use super::Vector;
use super::Vector2;

pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Default for Vector2f {
    fn default() -> Vector2f {
        Vector2f {
            x: 0f32,
            y: 0f32,
        }
    }
}

impl Vector<Vector2f, f32> for Vector2f {
    fn add(&mut self, v: &Vector2f) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self
    }

    fn add_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        dest.x = self.x + v.x;
        dest.y = self.y + v.y;
    }

    fn angle(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn angle_cos(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn distance(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn distance_squared(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn dot(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn fma_scalar(&mut self, a: f32, b: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn fma_scalar_into(&self, a: f32, b: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn fma_vector(&mut self, a: &Vector2f, b: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn fma_vector_into(&self, a: &Vector2f, b: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn length(&self) -> f32 {
        unimplemented!()
    }

    fn length_squared(&self) -> f32 {
        unimplemented!()
    }

    fn lerp(&mut self, other: &Vector2f, t: f32) -> &mut Self {
        unimplemented!()
    }

    fn lerp_into(&self, other: &Vector2f, t: f32, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn mul_scalar(&mut self, s: f32) -> &mut Self {
        unimplemented!()
    }

    fn mul_scalar_into(&self, s: f32, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn mul_vector(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn mul_vector_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn negate(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn negate_into(&self, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn normalize(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn normalize_into(&self, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn set(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn sub(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn sub_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn zero(&mut self) -> &mut Self {
        unimplemented!()
    }
}

impl Vector2<Vector2f, f32> for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Vector2f {
            x: x,
            y: y,
        }
    }

    fn add_components(&mut self, x: f32, y: f32) -> &mut Self {
        unimplemented!()
    }

    fn add_components_into(&self, x: f32, y: f32, dest: &mut Vector2f) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Vector;
    use super::super::Vector2;
    use super::Vector2f;

    #[test]
    fn test_new() {
        let a = Vector2f::new(1f32, 2f32);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
    }

    #[test]
    fn test_add() {
        let mut a = Vector2f::default();
        let b = Vector2f::new(1f32, 2f32);

        a.add(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);

        assert_eq!(b.x, 1f32);
        assert_eq!(b.y, 2f32);
    }

    #[test]
    fn test_add_into() {
        let a = Vector2f::new(1f32, 1f32);
        let b = Vector2f::new(1f32, 2f32);
        let mut c = Vector2f::default();

        a.add_into(&b, &mut c);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 1f32);

        assert_eq!(b.x, 1f32);
        assert_eq!(b.y, 2f32);

        assert_eq!(c.x, 2f32);
        assert_eq!(c.y, 3f32);
    }
}
