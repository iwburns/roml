use vector::Vector;
use vector::Vector3;
use ThreeTuple;

#[derive(Default)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<'a> From<&'a Vector3f> for Vector3f {
    fn from(other: &'a Vector3f) -> Vector3f {
        Vector3f {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

impl<'a> From<&'a f32> for Vector3f {
    fn from(other: &'a f32) -> Vector3f {
        Vector3f {
            x: *other,
            y: *other,
            z: *other,
        }
    }
}

impl<'a> From<&'a ThreeTuple<f32>> for Vector3f {
    fn from(other: &'a ThreeTuple<f32>) -> Vector3f {
        Vector3f {
            x: other.0,
            y: other.1,
            z: other.2,
        }
    }
}

impl Vector<f32> for Vector3f {
    fn add<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }

    fn add_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        dest.x = self.x + rhs.x;
        dest.y = self.y + rhs.y;
        dest.z = self.z + rhs.z;
    }

    fn angle<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        let v = Vector3f::from(rhs);
        let mut cos = self.angle_cos(rhs);
        cos = cos.min(1f32);
        cos = cos.max(-1f32);
        cos.acos()
    }

    fn angle_cos<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        let v = Vector3f::from(rhs);
        let self_len_squared = self.length_sq();
        let v_len_squared = v.length_sq();
        let dot = self.dot(rhs);
        dot / ((self_len_squared * v_len_squared).sqrt())
    }

    fn distance<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        self.distance_sq(rhs).sqrt()
    }

    fn distance_sq<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        let dz = self.z - rhs.z;
        (dx * dx) + (dy * dy) + (dz * dz)
    }

    fn dot<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        let v = Vector3f::from(rhs);
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    fn fma<'a, V>(&mut self, a: &'a V, b: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let a = Vector3f::from(a);
        let b = Vector3f::from(b);
        self.x = a.x.mul_add(b.x, self.x);
        self.y = a.y.mul_add(b.y, self.y);
        self.z = a.z.mul_add(b.z, self.z);
        self
    }

    fn fma_into<'a, V>(&self, a: &'a V, b: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let a = Vector3f::from(a);
        let b = Vector3f::from(b);
        dest.x = a.x.mul_add(b.x, self.x);
        dest.y = a.y.mul_add(b.y, self.y);
        dest.z = a.z.mul_add(b.z, self.z);
    }

    fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    fn length_sq(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn lerp<'a, V>(&mut self, other: &'a V, t: f32) -> &mut Self
        where Self: From<&'a V>
    {
        let other = Vector3f::from(other);
        self.x += (other.x - self.x) * t;
        self.y += (other.y - self.y) * t;
        self.z += (other.z - self.z) * t;
        self
    }

    fn lerp_into<'a, V>(&self, other: &'a V, t: f32, dest: &mut Self)
        where Self: From<&'a V>
    {
        let other = Vector3f::from(other);
        dest.x = self.x + (other.x - self.x) * t;
        dest.y = self.y + (other.y - self.y) * t;
        dest.z = self.z + (other.z - self.z) * t;
    }

    fn mul<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }

    fn mul_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        dest.x = self.x * rhs.x;
        dest.y = self.y * rhs.y;
        dest.z = self.z * rhs.z;
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    fn negate_into(&self, dest: &mut Self) {
        dest.x = -self.x;
        dest.y = -self.y;
        dest.z = -self.z;
    }

    fn normalize(&mut self) -> &mut Self {
        let inv_length = 1f32 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
        self.z *= inv_length;
        self
    }

    fn normalize_into(&self, dest: &mut Self) {
        let inv_length = 1f32 / self.length();
        dest.x = self.x * inv_length;
        dest.y = self.y * inv_length;
        dest.z = self.z * inv_length;
    }

    fn set<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        self.x = rhs.x;
        self.y = rhs.y;
        self.z = rhs.z;
        self
    }

    fn sub<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }

    fn sub_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        dest.x = self.x - rhs.x;
        dest.y = self.y - rhs.y;
        dest.z = self.z - rhs.z;
    }

    fn zero(&mut self) -> &mut Self {
        self.x = 0f32;
        self.y = 0f32;
        self.z = 0f32;
        self
    }
}

impl Vector3<f32> for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3f { x: x, y: y, z: z }
    }

    fn cross<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);

        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.z * rhs.x) - (self.x * rhs.z);
        let z = (self.x * rhs.y) - (self.y * rhs.x);

        self.x = x;
        self.y = y;
        self.z = z;

        self
    }

    fn cross_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector3f::from(rhs);
        dest.x = (self.y * rhs.z) - (self.z * rhs.y);
        dest.y = (self.z * rhs.x) - (self.x * rhs.z);
        dest.z = (self.x * rhs.y) - (self.y * rhs.x);
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
    fn test_from_vector3f() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::from(&a);

        assert_eq!(b.x, 1f32);
        assert_eq!(b.y, 2f32);
        assert_eq!(b.z, 3f32);
    }

    #[test]
    fn test_from_three_tuple() {
        let a = Vector3f::from(&(1f32, 2f32, 3f32));

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
        assert_eq!(a.z, 3f32);
    }

    #[test]
    fn test_from_f32() {
        let a = Vector3f::from(&1f32);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 1f32);
        assert_eq!(a.z, 1f32);
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
    fn test_angle() {
        let a = Vector3f::new(1f32, 0f32, 0f32);
        let b = Vector3f::new(0f32, 1f32, 0f32);

        let target_angle = 90f32.to_radians();
        let angle = a.angle(&b);

        assert!((target_angle - angle).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_angle_cos() {
        let a = Vector3f::new(1f32, 0f32, 0f32);
        let b = Vector3f::new(0f32, 1f32, 0f32);

        let target_angle_cos = 90f32.to_radians().cos();
        let angle_cos = a.angle_cos(&b);

        assert!((target_angle_cos - angle_cos).abs() <= std::f32::EPSILON);
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
    fn test_distance_sq() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(-1f32, 2f32, 3f32);

        let target_distance_sq = 4f32; // target distance is 2
        let distance_sq = a.distance_sq(&b);

        assert_eq!(target_distance_sq, distance_sq);
    }

    #[test]
    fn test_dot() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(1f32, 2f32, 3f32);

        let target_dot = 14f32;
        let dot = a.dot(&b);

        assert_eq!(target_dot, dot);
    }

    #[test]
    fn test_fma() {
        let mut a = Vector3f::new(1f32, 1f32, 1f32);
        let b = Vector3f::new(2f32, 3f32, 4f32);
        let c = Vector3f::new(2f32, 3f32, 4f32);

        a.fma(&b, &c);

        assert_eq!(a.x, 5f32);
        assert_eq!(a.y, 10f32);
        assert_eq!(a.z, 17f32);
    }

    #[test]
    fn test_fma_into() {
        let a = Vector3f::new(1f32, 1f32, 1f32);
        let b = Vector3f::new(2f32, 3f32, 4f32);
        let c = Vector3f::new(2f32, 3f32, 4f32);
        let mut d = Vector3f::new(0f32, 0f32, 0f32);

        a.fma_into(&b, &c, &mut d);

        assert_eq!(d.x, 5f32);
        assert_eq!(d.y, 10f32);
        assert_eq!(d.z, 17f32);
    }

    #[test]
    fn test_length() {
        let a = Vector3f::new(1f32, 2f32, 2f32);

        let target_length = 3f32;
        let length = a.length();

        assert_eq!(target_length, length);
    }

    #[test]
    fn test_length_squared() {
        let a = Vector3f::new(1f32, 2f32, 3f32);

        let target_length_sq = 14f32;
        let length_sq = a.length_sq();

        assert_eq!(target_length_sq, length_sq);
    }

    #[test]
    fn test_lerp() {
        let mut a = Vector3f::new(1f32, 0f32, 0f32);
        let b = Vector3f::new(2f32, 2f32, 3f32);

        a.lerp(&b, 0.5f32);

        assert_eq!(a.x, 1.5f32);
        assert_eq!(a.y, 1.0f32);
        assert_eq!(a.z, 1.5f32);
    }

    #[test]
    fn test_lerp_into() {
        let a = Vector3f::new(1f32, 0f32, 0f32);
        let b = Vector3f::new(2f32, 2f32, 3f32);
        let mut c = Vector3f::new(0f32, 0f32, 0f32);

        a.lerp_into(&b, 0.5f32, &mut c);

        assert_eq!(c.x, 1.5f32);
        assert_eq!(c.y, 1.0f32);
        assert_eq!(c.z, 1.5f32);
    }

    #[test]
    fn test_mul() {
        let mut a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(1f32, 2f32, 3f32);

        a.mul(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 4f32);
        assert_eq!(a.z, 9f32);
    }

    #[test]
    fn test_mul_into() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(1f32, 2f32, 3f32);
        let mut c = Vector3f::new(0f32, 0f32, 0f32);

        a.mul_into(&b, &mut c);

        assert_eq!(c.x, 1f32);
        assert_eq!(c.y, 4f32);
        assert_eq!(c.z, 9f32);
    }

    #[test]
    fn test_negate() {
        let mut a = Vector3f::new(1f32, 2f32, 3f32);

        a.negate();

        assert_eq!(a.x, -1f32);
        assert_eq!(a.y, -2f32);
        assert_eq!(a.z, -3f32);
    }

    #[test]
    fn test_negate_into() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let mut b = Vector3f::new(0f32, 0f32, 0f32);

        a.negate_into(&mut b);

        assert_eq!(b.x, -1f32);
        assert_eq!(b.y, -2f32);
        assert_eq!(b.z, -3f32);
    }

    #[test]
    fn test_normalize() {
        let mut a = Vector3f::new(1f32, 2f32, 3f32);

        a.normalize();

        assert!((1f32 - a.length()).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_normalize_into() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let mut b = Vector3f::new(0f32, 0f32, 0f32);

        a.normalize_into(&mut b);

        assert!((1f32 - b.length()).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_set() {
        let mut a = Vector3f::new(0f32, 0f32, 0f32);
        let b = Vector3f::new(1f32, 2f32, 3f32);

        a.set(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
        assert_eq!(a.z, 3f32);
    }

    #[test]
    fn test_sub() {
        let mut a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(0f32, 1f32, 2f32);

        a.sub(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 1f32);
        assert_eq!(a.z, 1f32);
    }

    #[test]
    fn test_sub_into() {
        let a = Vector3f::new(1f32, 2f32, 3f32);
        let b = Vector3f::new(0f32, 1f32, 2f32);
        let mut c = Vector3f::new(0f32, 0f32, 0f32);

        a.sub_into(&b, &mut c);

        assert_eq!(c.x, 1f32);
        assert_eq!(c.y, 1f32);
        assert_eq!(c.z, 1f32);
    }

    #[test]
    fn test_zero() {
        let mut a = Vector3f::new(1f32, 2f32, 3f32);

        a.zero();

        assert_eq!(a.x, 0f32);
        assert_eq!(a.y, 0f32);
        assert_eq!(a.z, 0f32);
    }

    #[test]
    fn test_cross() {
        let mut a = Vector3f::new(1f32, 0f32, 0f32);
        let b = Vector3f::new(0f32, 1f32, 0f32);

        a.cross(&b);

        assert_eq!(a.x, 0f32);
        assert_eq!(a.y, 0f32);
        assert_eq!(a.z, 1f32);
    }

    #[test]
    fn test_cross_into() {
        let a = Vector3f::new(1f32, 0f32, 0f32);
        let b = Vector3f::new(0f32, 1f32, 0f32);
        let mut c = Vector3f::new(0f32, 0f32, 0f32);


        a.cross_into(&b, &mut c);

        assert_eq!(c.x, 0f32);
        assert_eq!(c.y, 0f32);
        assert_eq!(c.z, 1f32);
    }
}
