use vector::Vector;
use vector::Vector4;
use FourTuple;

pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4<f32> for Vector4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4f {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl Vector<f32> for Vector4f {
    fn add<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
        self
    }

    fn add_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        dest.x = self.x + rhs.x;
        dest.y = self.y + rhs.y;
        dest.z = self.z + rhs.z;
        dest.w = self.w + rhs.w;
    }

    fn angle<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn angle_cos<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn distance<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        self.distance_sq(rhs).sqrt()
    }

    fn distance_sq<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        let dz = self.z - rhs.z;
        let dw = self.w - rhs.w;
        (dx * dx) + (dy * dy) + (dz * dz) + (dw * dw)
    }

    fn dot<'a, V>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        let v = Vector4f::from(rhs);
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z) + (self.w * v.w)
    }

    fn fma<'a, V>(&mut self, a: &'a V, b: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn fma_into<'a, V>(&self, a: &'a V, b: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    fn length_sq(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
    }

    fn lerp<'a, V>(&mut self, other: &'a V, t: f32) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn lerp_into<'a, V>(&self, other: &'a V, t: f32, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn mul<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
        self
    }

    fn mul_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        dest.x = self.x * rhs.x;
        dest.y = self.y * rhs.y;
        dest.z = self.z * rhs.z;
        dest.w = self.w * rhs.w;
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
        self
    }

    fn negate_into(&self, dest: &mut Self) {
        dest.x = -self.x;
        dest.y = -self.y;
        dest.z = -self.z;
        dest.w = -self.w;
    }

    fn normalize(&mut self) -> &mut Self {
        let inv_length = 1f32 / self.length();
        self.x *= inv_length;
        self.y *= inv_length;
        self.z *= inv_length;
        self.w *= inv_length;
        self
    }

    fn normalize_into(&self, dest: &mut Self) {
        let inv_length = 1f32 / self.length();
        dest.x = self.x * inv_length;
        dest.y = self.y * inv_length;
        dest.z = self.z * inv_length;
        dest.w = self.w * inv_length;
    }

    fn set<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        self.x = rhs.x;
        self.y = rhs.y;
        self.z = rhs.z;
        self.w = rhs.w;
        self
    }

    fn sub<'a, V>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
        self
    }

    fn sub_into<'a, V>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        let rhs = Vector4f::from(rhs);
        dest.x = self.x - rhs.x;
        dest.y = self.y - rhs.y;
        dest.z = self.z - rhs.z;
        dest.w = self.w - rhs.w;
    }

    fn zero(&mut self) -> &mut Self {
        self.x = 0f32;
        self.y = 0f32;
        self.z = 0f32;
        self.w = 0f32;
        self
    }
}

impl<'a> From<&'a Vector4f> for Vector4f {
    fn from(other: &'a Vector4f) -> Self {
        Vector4f {
            x: other.x,
            y: other.y,
            z: other.z,
            w: other.w,
        }
    }
}

impl<'a> From<&'a FourTuple<f32>> for Vector4f {
    fn from(other: &'a FourTuple<f32>) -> Self {
        Vector4f {
            x: other.0,
            y: other.1,
            z: other.2,
            w: other.3,
        }
    }
}

impl<'a> From<&'a f32> for Vector4f {
    fn from(other: &'a f32) -> Self {
        Vector4f {
            x: *other,
            y: *other,
            z: *other,
            w: *other,
        }
    }
}

#[cfg(test)]
mod tests {
    use vector::Vector;
    use vector::Vector4;
    use vector::vector4f::Vector4f;

    use std;

    #[test]
    fn test_new() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
        assert_eq!(a.z, 3f32);
        assert_eq!(a.w, 4f32);
    }

    #[test]
    fn test_from_vector() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::from(&a);

        assert_eq!(b.x, 1f32);
        assert_eq!(b.y, 2f32);
        assert_eq!(b.z, 3f32);
        assert_eq!(b.w, 4f32);
    }

    #[test]
    fn test_from_tuple() {
        let a = Vector4f::from(&(1f32, 2f32, 3f32, 4f32));

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
        assert_eq!(a.z, 3f32);
        assert_eq!(a.w, 4f32);
    }

    #[test]
    fn test_from_f32() {
        let a = Vector4f::from(&1f32);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 1f32);
        assert_eq!(a.z, 1f32);
        assert_eq!(a.w, 1f32);
    }

    #[test]
    fn test_add() {
        let mut a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        a.add(&b);

        assert_eq!(a.x, 2f32);
        assert_eq!(a.y, 4f32);
        assert_eq!(a.z, 6f32);
        assert_eq!(a.w, 8f32);
    }

    #[test]
    fn test_add_into() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let mut c = Vector4f::new(0f32, 0f32, 0f32, 0f32);

        a.add_into(&b, &mut c);

        assert_eq!(c.x, 2f32);
        assert_eq!(c.y, 4f32);
        assert_eq!(c.z, 6f32);
        assert_eq!(c.w, 8f32);
    }
//
//    #[test]
//    fn test_angle() {
//        let a = Vector3f::new(1f32, 0f32, 0f32);
//        let b = Vector3f::new(0f32, 1f32, 0f32);
//
//        let target_angle = 90f32.to_radians();
//        let angle = a.angle(&b);
//
//        assert!((target_angle - angle).abs() <= std::f32::EPSILON);
//    }
//
//    #[test]
//    fn test_angle_cos() {
//        let a = Vector3f::new(1f32, 0f32, 0f32);
//        let b = Vector3f::new(0f32, 1f32, 0f32);
//
//        let target_angle_cos = 90f32.to_radians().cos();
//        let angle_cos = a.angle_cos(&b);
//
//        assert!((target_angle_cos - angle_cos).abs() <= std::f32::EPSILON);
//    }
//
    #[test]
    fn test_distance() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(-1f32, 2f32, 3f32, 4f32);

        let target_distance = 2f32;
        let distance = a.distance(&b);

        assert_eq!(target_distance, distance);
    }

    #[test]
    fn test_distance_sq() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(-1f32, 2f32, 3f32, 4f32);

        let target_distance_sq = 4f32; // target distance is 2
        let distance_sq = a.distance_sq(&b);

        assert_eq!(target_distance_sq, distance_sq);
    }

    #[test]
    fn test_dot() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        let target_dot = 30f32;
        let dot = a.dot(&b);

        assert_eq!(target_dot, dot);
    }

//    #[test]
//    fn test_fma() {
//        let mut a = Vector3f::new(1f32, 1f32, 1f32);
//        let b = Vector3f::new(2f32, 3f32, 4f32);
//        let c = Vector3f::new(2f32, 3f32, 4f32);
//
//        a.fma(&b, &c);
//
//        assert_eq!(a.x, 5f32);
//        assert_eq!(a.y, 10f32);
//        assert_eq!(a.z, 17f32);
//    }
//
//    #[test]
//    fn test_fma_into() {
//        let a = Vector3f::new(1f32, 1f32, 1f32);
//        let b = Vector3f::new(2f32, 3f32, 4f32);
//        let c = Vector3f::new(2f32, 3f32, 4f32);
//        let mut d = Vector3f::new(0f32, 0f32, 0f32);
//
//        a.fma_into(&b, &c, &mut d);
//
//        assert_eq!(d.x, 5f32);
//        assert_eq!(d.y, 10f32);
//        assert_eq!(d.z, 17f32);
//    }
//
    #[test]
    fn test_length() {
        let a = Vector4f::new(1f32, 0f32, 2f32, 2f32);

        let target_length = 3f32;
        let length = a.length();

        assert_eq!(target_length, length);
    }

    #[test]
    fn test_length_squared() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        let target_length_sq = 30f32;
        let length_sq = a.length_sq();

        assert_eq!(target_length_sq, length_sq);
    }

//    #[test]
//    fn test_lerp() {
//        let mut a = Vector3f::new(1f32, 0f32, 0f32);
//        let b = Vector3f::new(2f32, 2f32, 3f32);
//
//        a.lerp(&b, 0.5f32);
//
//        assert_eq!(a.x, 1.5f32);
//        assert_eq!(a.y, 1.0f32);
//        assert_eq!(a.z, 1.5f32);
//    }
//
//    #[test]
//    fn test_lerp_into() {
//        let a = Vector3f::new(1f32, 0f32, 0f32);
//        let b = Vector3f::new(2f32, 2f32, 3f32);
//        let mut c = Vector3f::new(0f32, 0f32, 0f32);
//
//        a.lerp_into(&b, 0.5f32, &mut c);
//
//        assert_eq!(c.x, 1.5f32);
//        assert_eq!(c.y, 1.0f32);
//        assert_eq!(c.z, 1.5f32);
//    }
//
    #[test]
    fn test_mul() {
        let mut a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        a.mul(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 4f32);
        assert_eq!(a.z, 9f32);
        assert_eq!(a.w, 16f32);
    }

    #[test]
    fn test_mul_into() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let mut c = Vector4f::new(0f32, 0f32, 0f32, 0f32);

        a.mul_into(&b, &mut c);

        assert_eq!(c.x, 1f32);
        assert_eq!(c.y, 4f32);
        assert_eq!(c.z, 9f32);
        assert_eq!(c.w, 16f32);
    }

    #[test]
    fn test_negate() {
        let mut a = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        a.negate();

        assert_eq!(a.x, -1f32);
        assert_eq!(a.y, -2f32);
        assert_eq!(a.z, -3f32);
        assert_eq!(a.w, -4f32);
    }

    #[test]
    fn test_negate_into() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let mut b = Vector4f::new(0f32, 0f32, 0f32, 0f32);

        a.negate_into(&mut b);

        assert_eq!(b.x, -1f32);
        assert_eq!(b.y, -2f32);
        assert_eq!(b.z, -3f32);
        assert_eq!(b.w, -4f32);
    }

    #[test]
    fn test_normalize() {
        let mut a = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        a.normalize();

        assert!((1f32 - a.length()).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_normalize_into() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let mut b = Vector4f::new(0f32, 0f32, 0f32, 0f32);

        a.normalize_into(&mut b);

        assert!((1f32 - b.length()).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn test_set() {
        let mut a = Vector4f::new(0f32, 0f32, 0f32, 0f32);
        let b = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        a.set(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 2f32);
        assert_eq!(a.z, 3f32);
        assert_eq!(a.w, 4f32);
    }

    #[test]
    fn test_sub() {
        let mut a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(0f32, 1f32, 2f32, 3f32);

        a.sub(&b);

        assert_eq!(a.x, 1f32);
        assert_eq!(a.y, 1f32);
        assert_eq!(a.z, 1f32);
        assert_eq!(a.w, 1f32);
    }

    #[test]
    fn test_sub_into() {
        let a = Vector4f::new(1f32, 2f32, 3f32, 4f32);
        let b = Vector4f::new(0f32, 1f32, 2f32, 3f32);
        let mut c = Vector4f::new(0f32, 0f32, 0f32, 0f32);

        a.sub_into(&b, &mut c);

        assert_eq!(c.x, 1f32);
        assert_eq!(c.y, 1f32);
        assert_eq!(c.z, 1f32);
        assert_eq!(c.w, 1f32);
    }

    #[test]
    fn test_zero() {
        let mut a = Vector4f::new(1f32, 2f32, 3f32, 4f32);

        a.zero();

        assert_eq!(a.x, 0f32);
        assert_eq!(a.y, 0f32);
        assert_eq!(a.z, 0f32);
        assert_eq!(a.w, 0f32);
    }
}

