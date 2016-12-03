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
        let dot = self.dot(v);
        let det = (self.x * v.y) - (self.y * v.x);
        det.atan2(dot)
    }

    fn angle_cos(&self, v: &Vector2f) -> f32 {
        let self_len_squared = self.length_squared();
        let v_len_squared = v.length_squared();
        let dot = self.dot(v);
        dot / ((self_len_squared * v_len_squared).sqrt())
    }

    fn distance(&self, v: &Vector2f) -> f32 {
        self.distance_squared(v).sqrt()
    }

    fn distance_squared(&self, v: &Vector2f) -> f32 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        (dx * dx) + (dy * dy)
    }

    fn dot(&self, v: &Vector2f) -> f32 {
        (self.x * v.x) + (self.y * v.y)
    }

    fn fma_scalar(&mut self, a: f32, b: &Vector2f) -> &mut Self {
        self.x = a.mul_add(b.x, self.x);
        self.y = a.mul_add(b.y, self.y);
        self
    }

    fn fma_scalar_into(&self, a: f32, b: &Vector2f, dest: &mut Vector2f) {
        dest.x = a.mul_add(b.x, self.x);
        dest.y = a.mul_add(b.y, self.y);
    }

    fn fma_vector(&mut self, a: &Vector2f, b: &Vector2f) -> &mut Self {
        self.x = a.x.mul_add(b.x, self.x);
        self.y = a.y.mul_add(b.y, self.y);
        self
    }

    fn fma_vector_into(&self, a: &Vector2f, b: &Vector2f, dest: &mut Vector2f) {
        dest.x = a.x.mul_add(b.x, self.x);
        dest.y = a.y.mul_add(b.y, self.y);
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    fn lerp(&mut self, other: &Vector2f, t: f32) -> &mut Self {
        self.x += (other.x - self.x) * t;
        self.y += (other.y - self.y) * t;
        self
    }

    fn lerp_into(&self, other: &Vector2f, t: f32, dest: &mut Vector2f) {
        dest.x = self.x + (other.x - self.x) * t;
        dest.y = self.y + (other.y - self.y) * t;
    }

    fn mul_scalar(&mut self, s: f32) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self
    }

    fn mul_scalar_into(&self, s: f32, dest: &mut Vector2f) {
        dest.x = self.x * s;
        dest.y = self.y * s;
    }

    fn mul_vector(&mut self, v: &Vector2f) -> &mut Self {
        self.x *= v.x;
        self.y *= v.y;
        self
    }

    fn mul_vector_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        dest.x = self.x * v.x;
        dest.y = self.y * v.y;
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }

    fn negate_into(&self, dest: &mut Vector2f) {
        dest.x = -self.x;
        dest.y = -self.y;
    }

    fn normalize(&mut self) -> &mut Self {
        let inv_length = 1f32 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
        self
    }

    fn normalize_into(&self, dest: &mut Vector2f) {
        let inv_length = 1f32 / self.length();
        dest.x = self.x * inv_length;
        dest.y = self.y * inv_length;
    }

    fn set(&mut self, v: &Vector2f) -> &mut Self {
        self.x = v.x;
        self.y = v.y;
        self
    }

    fn sub(&mut self, v: &Vector2f) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self
    }

    fn sub_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        dest.x = self.x - v.x;
        dest.y = self.y - v.y;
    }

    fn zero(&mut self) -> &mut Self {
        self.x = 0f32;
        self.y = 0f32;
        self
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
        self.x += x;
        self.y += y;
        self
    }

    fn add_components_into(&self, x: f32, y: f32, dest: &mut Vector2f) {
        dest.x = self.x + x;
        dest.y = self.y + y;
    }
}

#[cfg(test)]
mod tests {
    use super::super::Vector;
    use super::super::Vector2;
    use super::Vector2f;

    use std;

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
    }

    #[test]
    fn test_add_into() {
        let a = Vector2f::new(1f32, 1f32);
        let b = Vector2f::new(1f32, 2f32);
        let mut c = Vector2f::default();

        a.add_into(&b, &mut c);

        assert_eq!(c.x, 2f32);
        assert_eq!(c.y, 3f32);
    }

    #[test]
    fn test_angle() {
        let a = Vector2f::new(1f32, 0f32);
        let b = Vector2f::new(0f32, 1f32);

        let target_angle = 90f32.to_radians();
        let angle = a.angle(&b);

        assert!((target_angle - angle).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_angle_cos() {
        let a = Vector2f::new(1f32, 0f32);
        let b = Vector2f::new(0f32, 1f32);

        let target_angle_cos = 90f32.to_radians().cos();
        let angle_cos = a.angle_cos(&b);

        assert!((target_angle_cos - angle_cos).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_distance() {
        let a = Vector2f::new(2f32, 1f32);
        let b = Vector2f::new(0f32, 1f32);

        let target_distance = 2f32;
        let distance = a.distance(&b);

        assert_eq!(target_distance, distance);
    }

    #[test]
    fn test_distance_squared() {
        let a = Vector2f::new(2f32, 1f32);
        let b = Vector2f::new(0f32, 1f32);

        let target_distance_sq = 4f32; //distance is 2
        let distance_sq = a.distance_squared(&b);

        assert_eq!(target_distance_sq, distance_sq);
    }

    #[test]
    fn test_dot() {
        let a = Vector2f::new(2f32, 1f32);
        let b = Vector2f::new(1f32, 1f32);

        let target_dot = 3f32;
        let dot = a.dot(&b);

        assert_eq!(target_dot, dot);
    }

    #[test]
    fn test_fma_scalar() {
        let mut a = Vector2f::new(1f32, 1f32);
        let b = 2f32;
        let c = Vector2f::new(2f32, 3f32);

        a.fma_scalar(b, &c);

        assert_eq!(a.x, 5f32);
        assert_eq!(a.y, 7f32);
    }

    #[test]
    fn test_fma_scalar_into() {
        let a = Vector2f::new(1f32, 1f32);
        let b = 2f32;
        let c = Vector2f::new(2f32, 3f32);
        let mut d = Vector2f::new(0f32, 0f32);

        a.fma_scalar_into(b, &c, &mut d);

        assert_eq!(d.x, 5f32);
        assert_eq!(d.y, 7f32);
    }

    #[test]
    fn test_fma_vector() {
        let mut a = Vector2f::new(1f32, 1f32);
        let b = Vector2f::new(2f32, 3f32);
        let c = Vector2f::new(2f32, 3f32);

        a.fma_vector(&b, &c);

        assert_eq!(a.x, 5f32);
        assert_eq!(a.y, 10f32);
    }

    #[test]
    fn test_fma_vector_into() {
        let a = Vector2f::new(1f32, 1f32);
        let b = Vector2f::new(2f32, 3f32);
        let c = Vector2f::new(2f32, 3f32);
        let mut d = Vector2f::new(0f32, 0f32);

        a.fma_vector_into(&b, &c, &mut d);

        assert_eq!(d.x, 5f32);
        assert_eq!(d.y, 10f32);
    }

    #[test]
    fn test_length() {
        let a = Vector2f::new(3f32, 4f32);

        let target_length = 5f32; //length should be 5 because of the 3:4:5 triangle
        let length = a.length();

        assert_eq!(target_length, length);
    }

    #[test]
    fn test_length_squared() {
        let a = Vector2f::new(3f32, 4f32);

        let target_length_sq = 25f32; //length should be 5 because of the 3:4:5 triangle
        let length_sq = a.length_squared();

        assert_eq!(target_length_sq, length_sq);
    }

    #[test]
    fn test_lerp() {
        let mut a = Vector2f::new(1f32, 0f32);
        let b = Vector2f::new(2f32, 2f32);

        a.lerp(&b, 0.5f32);

        assert_eq!(a.x, 1.5f32);
        assert_eq!(a.y, 1.0f32);
    }

    #[test]
    fn test_lerp_into() {
        let a = Vector2f::new(1f32, 0f32);
        let b = Vector2f::new(2f32, 2f32);
        let mut c = Vector2f::new(0f32, 0f32);

        a.lerp_into(&b, 0.5f32, &mut c);

        assert_eq!(c.x, 1.5f32);
        assert_eq!(c.y, 1.0f32);
    }

    #[test]
    fn test_mul_scalar() {
        let mut a = Vector2f::new(1f32, 2f32);
        let b = 2f32;

        a.mul_scalar(b);

        assert_eq!(a.x, 2f32);
        assert_eq!(a.y, 4f32);
    }

    #[test]
    fn test_mul_scalar_into() {
        let a = Vector2f::new(1f32, 2f32);
        let b = 2f32;
        let mut c = Vector2::new(0f32, 0f32);

        a.mul_scalar_into(b, &mut c);

        assert_eq!(c.x, 2f32);
        assert_eq!(c.y, 4f32);
    }

    #[test]
    fn test_mul_vector() {
        let mut a = Vector2f::new(1f32, 2f32);
        let b = Vector2f::new(2f32, 3f32);

        a.mul_vector(&b);

        assert_eq!(a.x, 2f32);
        assert_eq!(a.y, 6f32);
    }

    #[test]
    fn test_mul_vector_into() {
        let a = Vector2f::new(1f32, 2f32);
        let b = Vector2f::new(2f32, 3f32);
        let mut c = Vector2::new(0f32, 0f32);

        a.mul_vector_into(&b, &mut c);

        assert_eq!(c.x, 2f32);
        assert_eq!(c.y, 6f32);
    }

    #[test]
    fn test_negate() {
        let mut a = Vector2f::new(1f32, 2f32);

        a.negate();

        assert_eq!(a.x, -1f32);
        assert_eq!(a.y, -2f32);
    }

    #[test]
    fn test_negate_into() {
        let a = Vector2f::new(1f32, 2f32);
        let mut b = Vector2f::new(0f32, 0f32);

        a.negate_into(&mut b);

        assert_eq!(b.x, -1f32);
        assert_eq!(b.y, -2f32);
    }

    #[test]
    fn test_normalize() {
        let mut a = Vector2f::new(1f32, 2f32);

        a.normalize();

        assert!((1f32 - a.length()).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_normalize_into() {
        let a = Vector2f::new(1f32, 2f32);
        let mut b = Vector2f::new(0f32, 0f32);

        a.normalize_into(&mut b);

        assert!((1f32 - b.length()).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_set() {
        let mut a = Vector2f::new(0f32, 0f32);
        let b = Vector2f::new(1f32, 2f32);

        a.set(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
    }

    #[test]
    fn test_sub() {
        let mut a = Vector2f::new(2f32, 5f32);
        let b = Vector2f::new(1f32, 2f32);

        a.sub(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 3f32);
    }

    #[test]
    fn test_sub_into() {
        let a = Vector2f::new(2f32, 5f32);
        let b = Vector2f::new(1f32, 2f32);
        let mut c = Vector2f::new(0f32, 0f32);

        a.sub_into(&b, &mut c);

        assert_eq!(c.x, 1f32);
        assert_eq!(c.y, 3f32);
    }

    #[test]
    fn test_zero() {
        let mut a = Vector2f::new(1f32, 2f32);

        a.zero();

        assert_eq!(a.x, 0f32);
        assert_eq!(a.y, 0f32);
    }

    #[test]
    fn test_add_components() {
        let mut a = Vector2f::new(1f32, 2f32);

        a.add_components(2f32, 3f32);

        assert_eq!(a.x, 3f32);
        assert_eq!(a.y, 5f32);
    }

    #[test]
    fn test_add_components_into() {
        let a = Vector2f::new(1f32, 2f32);
        let mut b = Vector2f::new(0f32, 0f32);

        a.add_components_into(2f32, 3f32, &mut b);

        assert_eq!(b.x, 3f32);
        assert_eq!(b.y, 5f32);
    }
}
