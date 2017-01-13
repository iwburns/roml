use vector::ReadVector2;
use vector::Vector2;
use TwoTuple;

#[derive(Default)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl ReadVector2<f32> for Vector2f {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
}

impl ReadVector2<f32> for f32 {
    fn x(&self) -> f32 {
        *self
    }
    fn y(&self) -> f32 {
        *self
    }
}

impl ReadVector2<f32> for TwoTuple<f32> {
    fn x(&self) -> f32 {
        self.0
    }
    fn y(&self) -> f32 {
        self.1
    }
}

impl Vector2<f32> for Vector2f {
    fn new(x: f32, y: f32) -> Self {
        Vector2f {
            x: x,
            y: y,
        }
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn add<V>(&mut self, rhs: &V) -> &mut Self
        where V: ReadVector2<f32>
    {
        self.x += rhs.x();
        self.y += rhs.y();
        self
    }

    fn add_into<V>(&self, rhs: &V, dest: &mut Self)
        where V: ReadVector2<f32>
    {
        dest.x = self.x + rhs.x();
        dest.y = self.y + rhs.y();
    }

    fn angle<V>(&self, rhs: &V) -> f32
        where V: ReadVector2<f32>
    {
        let dot = self.dot(rhs);
        let det = (self.x * rhs.y()) - (self.y * rhs.x());
        det.atan2(dot)
    }

    fn angle_cos<V>(&self, rhs: &V) -> f32
        where V: ReadVector2<f32>
    {
        let self_len_squared = self.length_sq();
        let v_len_squared = Vector2f::new(rhs.x(), rhs.y()).length_sq();
        let dot = self.dot(rhs);
        dot / ((self_len_squared * v_len_squared).sqrt())
    }

    fn distance<V>(&self, rhs: &V) -> f32
        where V: ReadVector2<f32>
    {
        self.distance_sq(rhs).sqrt()
    }

    fn distance_sq<V>(&self, rhs: &V) -> f32
        where V: ReadVector2<f32>
    {
        let dx = self.x - rhs.x();
        let dy = self.y - rhs.y();
        (dx * dx) + (dy * dy)
    }

    fn dot<V>(&self, rhs: &V) -> f32
        where V: ReadVector2<f32>
    {
        (self.x * rhs.x()) + (self.y * rhs.y())
    }

    fn fma<V>(&mut self, a: &V, b: &V) -> &mut Self
        where V: ReadVector2<f32>
    {
        self.x = a.x().mul_add(b.x(), self.x);
        self.y = a.y().mul_add(b.y(), self.y);
        self
    }

    fn fma_into<V>(&self, a: &V, b: &V, dest: &mut Self)
        where V: ReadVector2<f32>
    {
        dest.x = a.x().mul_add(b.x(), self.x);
        dest.y = a.y().mul_add(b.y(), self.y);
    }

    fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    fn length_sq(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    fn lerp<V>(&mut self, other: &V, t: f32) -> &mut Self
        where V: ReadVector2<f32>
    {
        self.x += (other.x() - self.x) * t;
        self.y += (other.y() - self.y) * t;
        self
    }

    fn lerp_into<V>(&self, other: &V, t: f32, dest: &mut Self)
        where V: ReadVector2<f32>
    {
        dest.x = self.x + (other.x() - self.x) * t;
        dest.y = self.y + (other.y() - self.y) * t;
    }

    fn mul<V>(&mut self, rhs: &V) -> &mut Self
        where V: ReadVector2<f32>
    {
        self.x *= rhs.x();
        self.y *= rhs.y();
        self
    }

    fn mul_into<V>(&self, rhs: &V, dest: &mut Self)
        where V: ReadVector2<f32>
    {
        dest.x = self.x * rhs.x();
        dest.y = self.y * rhs.y();
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }

    fn negate_into(&self, dest: &mut Self) {
        dest.x = -self.x;
        dest.y = -self.y;
    }

    fn normalize(&mut self) -> &mut Self {
        let inv_length = 1f32 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
        self
    }

    fn normalize_into(&self, dest: &mut Self) {
        let inv_length = 1f32 / self.length();
        dest.x = self.x * inv_length;
        dest.y = self.y * inv_length;
    }

    fn set<V>(&mut self, rhs: &V) -> &mut Self
        where V: ReadVector2<f32>
    {
        self.x = rhs.x();
        self.y = rhs.y();
        self
    }

    fn sub<V>(&mut self, rhs: &V) -> &mut Self
        where V: ReadVector2<f32>
    {
        self.x -= rhs.x();
        self.y -= rhs.y();
        self
    }

    fn sub_into<V>(&self, rhs: &V, dest: &mut Self)
        where V: ReadVector2<f32>
    {
        dest.x = self.x - rhs.x();
        dest.y = self.y - rhs.y();
    }

    fn zero(&mut self) -> &mut Self {
        self.x = 0f32;
        self.y = 0f32;
        self
    }
}

#[cfg(test)]
mod tests {
    use vector::Vector2;
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
    fn test_add_float() {
        let mut a = Vector2f::default();
        let b = 1f32;

        a.add(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 1f32);
    }

    #[test]
    fn test_add_tuple() {
        let mut a = Vector2f::default();
        let b = (1f32, 2f32);

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
    fn test_distance_sq() {
        let a = Vector2f::new(2f32, 1f32);
        let b = Vector2f::new(0f32, 1f32);

        let target_distance_sq = 4f32; //distance is 2
        let distance_sq = a.distance_sq(&b);

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
    fn test_fma() {
        let mut a = Vector2f::new(1f32, 1f32);
        let b = Vector2f::new(2f32, 3f32);
        let c = Vector2f::new(2f32, 3f32);

        a.fma(&b, &c);

        assert_eq!(a.x, 5f32);
        assert_eq!(a.y, 10f32);
    }

    #[test]
    fn test_fma_into() {
        let a = Vector2f::new(1f32, 1f32);
        let b = Vector2f::new(2f32, 3f32);
        let c = Vector2f::new(2f32, 3f32);
        let mut d = Vector2f::new(0f32, 0f32);

        a.fma_into(&b, &c, &mut d);

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
    fn test_length_sq() {
        let a = Vector2f::new(3f32, 4f32);

        let target_length_sq = 25f32; //length should be 5 because of the 3:4:5 triangle
        let length_sq = a.length_sq();

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
    fn test_mul() {
        let mut a = Vector2f::new(1f32, 2f32);
        let b = Vector2f::new(2f32, 3f32);

        a.mul(&b);

        assert_eq!(a.x, 2f32);
        assert_eq!(a.y, 6f32);
    }

    #[test]
    fn test_mul_into() {
        let a = Vector2f::new(1f32, 2f32);
        let b = Vector2f::new(2f32, 3f32);
        let mut c = Vector2f::new(0f32, 0f32);

        a.mul_into(&b, &mut c);

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
}
