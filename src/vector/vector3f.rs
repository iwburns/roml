use super::Vector;
use super::Vector3;

pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector<f32> for Vector3f {
    fn add<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn add_into<'a, V: 'a>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn angle<'a, V: 'a>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn angle_cos<'a, V: 'a>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn distance<'a, V: 'a>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn distance_sq<'a, V: 'a>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn dot<'a, V: 'a>(&self, rhs: &'a V) -> f32
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn fma<'a, V: 'a>(&mut self, a: &'a V, b: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn fma_into<'a, V: 'a>(&self, a: &'a V, b: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn length(&self) -> f32 {
        unimplemented!()
    }

    fn length_squared(&self) -> f32 {
        unimplemented!()
    }

    fn lerp<'a, V: 'a>(&mut self, other: &'a V, t: f32) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn lerp_into<'a, V: 'a>(&self, other: &'a V, t: f32, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn mul<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn mul_into<'a, V: 'a>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn negate(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn negate_into(&self, dest: &mut Self) {
        unimplemented!()
    }

    fn normalize(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn normalize_into(&self, dest: &mut Self) {
        unimplemented!()
    }

    fn set<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn sub<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn sub_into<'a, V: 'a>(&self, rhs: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn zero(&mut self) -> &mut Self {
        unimplemented!()
    }
}

impl Vector3<f32> for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        unimplemented!()
    }

    fn cross<'a, V: 'a>(&mut self, v: &'a V) -> &mut Self
        where Self: From<&'a V>
    {
        unimplemented!()
    }

    fn cross_into<'a, V: 'a>(&self, v: &'a V, dest: &mut Self)
        where Self: From<&'a V>
    {
        unimplemented!()
    }
}

// #[cfg(test)]
// mod tests {
// use super::super::Vector;
// use super::super::Vector3;
// use super::Vector3f;
//
// use std;
//
// #[test]
// fn test_new() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
//
// assert_eq!(a.x, 1f32);
// assert_eq!(a.y, 2f32);
// assert_eq!(a.z, 3f32);
// }
//
// #[test]
// fn test_add() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(1f32, 2f32, 3f32);
//
// a.add(&b);
//
// assert_eq!(a.x, 2f32);
// assert_eq!(a.y, 4f32);
// assert_eq!(a.z, 6f32);
// }
//
// #[test]
// fn test_add_into() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(1f32, 2f32, 3f32);
// let mut c = Vector3f::new(0f32, 0f32, 0f32);
//
// a.add_into(&b, &mut c);
//
// assert_eq!(c.x, 2f32);
// assert_eq!(c.y, 4f32);
// assert_eq!(c.z, 6f32);
// }
//
// #[test]
// fn test_angle() {
// let a = Vector3f::new(1f32, 0f32, 0f32);
// let b = Vector3f::new(0f32, 1f32, 0f32);
//
// let target_angle = 90f32.to_radians();
// let angle = a.angle(&b);
//
// assert!((target_angle - angle).abs() <= std::f32::EPSILON);
// }
//
// #[test]
// fn test_angle_cos() {
// let a = Vector3f::new(1f32, 0f32, 0f32);
// let b = Vector3f::new(0f32, 1f32, 0f32);
//
// let target_angle_cos = 90f32.to_radians().cos();
// let angle_cos = a.angle_cos(&b);
//
// assert!((target_angle_cos - angle_cos).abs() <= std::f32::EPSILON);
// }
//
// #[test]
// fn test_distance() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(-1f32, 2f32, 3f32);
//
// let target_distance = 2f32;
// let distance = a.distance(&b);
//
// assert_eq!(target_distance, distance);
// }
//
// #[test]
// fn test_distance_squared() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(-1f32, 2f32, 3f32);
//
// let target_distance_sq = 4f32; // target distance is 2
// let distance_sq = a.distance_squared(&b);
//
// assert_eq!(target_distance_sq, distance_sq);
// }
//
// #[test]
// fn test_dot() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(1f32, 2f32, 3f32);
//
// let target_dot = 14f32;
// let dot = a.dot(&b);
//
// assert_eq!(target_dot, dot);
// }
//
// #[test]
// fn test_fma_scalar() {
// let mut a = Vector3f::new(1f32, 1f32, 1f32);
// let b = 2f32;
// let c = Vector3f::new(2f32, 3f32, 4f32);
//
// a.fma_scalar(b, &c);
//
// assert_eq!(a.x, 5f32);
// assert_eq!(a.y, 7f32);
// assert_eq!(a.z, 9f32);
// }
//
// #[test]
// fn test_fma_scalar_into() {
// let a = Vector3f::new(1f32, 1f32, 1f32);
// let b = 2f32;
// let c = Vector3f::new(2f32, 3f32, 4f32);
// let mut d = Vector3f::new(0f32, 0f32, 0f32);
//
// a.fma_scalar_into(b, &c, &mut d);
//
// assert_eq!(d.x, 5f32);
// assert_eq!(d.y, 7f32);
// assert_eq!(d.z, 9f32);
// }
//
// #[test]
// fn test_fma_vector() {
// let mut a = Vector3f::new(1f32, 1f32, 1f32);
// let b = Vector3f::new(2f32, 3f32, 4f32);
// let c = Vector3f::new(2f32, 3f32, 4f32);
//
// a.fma_vector(&b, &c);
//
// assert_eq!(a.x, 5f32);
// assert_eq!(a.y, 10f32);
// assert_eq!(a.z, 17f32);
// }
//
// #[test]
// fn test_fma_vector_into() {
// let a = Vector3f::new(1f32, 1f32, 1f32);
// let b = Vector3f::new(2f32, 3f32, 4f32);
// let c = Vector3f::new(2f32, 3f32, 4f32);
// let mut d = Vector3f::new(0f32, 0f32, 0f32);
//
// a.fma_vector_into(&b, &c, &mut d);
//
// assert_eq!(d.x, 5f32);
// assert_eq!(d.y, 10f32);
// assert_eq!(d.z, 17f32);
// }
//
// #[test]
// fn test_length() {
// let a = Vector3f::new(1f32, 2f32, 2f32);
//
// let target_length = 3f32;
// let length = a.length();
//
// assert_eq!(target_length, length);
// }
//
// #[test]
// fn test_length_squared() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
//
// let target_length_sq = 14f32;
// let length_sq = a.length_squared();
//
// assert_eq!(target_length_sq, length_sq);
// }
//
// #[test]
// fn test_lerp() {
// let mut a = Vector3f::new(1f32, 0f32, 0f32);
// let b = Vector3f::new(2f32, 2f32, 3f32);
//
// a.lerp(&b, 0.5f32);
//
// assert_eq!(a.x, 1.5f32);
// assert_eq!(a.y, 1.0f32);
// assert_eq!(a.z, 1.5f32);
// }
//
// #[test]
// fn test_lerp_into() {
// let a = Vector3f::new(1f32, 0f32, 0f32);
// let b = Vector3f::new(2f32, 2f32, 3f32);
// let mut c = Vector3f::new(0f32, 0f32, 0f32);
//
// a.lerp_into(&b, 0.5f32, &mut c);
//
// assert_eq!(c.x, 1.5f32);
// assert_eq!(c.y, 1.0f32);
// assert_eq!(c.z, 1.5f32);
// }
//
// #[test]
// fn test_mul_scalar() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
//
// a.mul_scalar(2f32);
//
// assert_eq!(a.x, 2f32);
// assert_eq!(a.y, 4f32);
// assert_eq!(a.z, 6f32);
// }
//
// #[test]
// fn test_mul_scalar_into() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let mut b = Vector3f::new(0f32, 0f32, 0f32);
//
// a.mul_scalar_into(2f32, &mut b);
//
// assert_eq!(b.x, 2f32);
// assert_eq!(b.y, 4f32);
// assert_eq!(b.z, 6f32);
// }
//
// #[test]
// fn test_mul_vector() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(1f32, 2f32, 3f32);
//
// a.mul_vector(&b);
//
// assert_eq!(a.x, 1f32);
// assert_eq!(a.y, 4f32);
// assert_eq!(a.z, 9f32);
// }
//
// #[test]
// fn test_mul_vector_into() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(1f32, 2f32, 3f32);
// let mut c = Vector3f::new(0f32, 0f32, 0f32);
//
// a.mul_vector_into(&b, &mut c);
//
// assert_eq!(c.x, 1f32);
// assert_eq!(c.y, 4f32);
// assert_eq!(c.z, 9f32);
// }
//
// #[test]
// fn test_negate() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
//
// a.negate();
//
// assert_eq!(a.x, -1f32);
// assert_eq!(a.y, -2f32);
// assert_eq!(a.z, -3f32);
// }
//
// #[test]
// fn test_negate_into() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let mut b = Vector3f::new(0f32, 0f32, 0f32);
//
// a.negate_into(&mut b);
//
// assert_eq!(b.x, -1f32);
// assert_eq!(b.y, -2f32);
// assert_eq!(b.z, -3f32);
// }
//
// #[test]
// fn test_normalize() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
//
// a.normalize();
//
// assert!((1f32 - a.length()).abs() <= std::f32::EPSILON);
// }
//
// #[test]
// fn test_normalize_into() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let mut b = Vector3f::new(0f32, 0f32, 0f32);
//
// a.normalize_into(&mut b);
//
// assert!((1f32 - b.length()).abs() <= std::f32::EPSILON);
// }
//
// #[test]
// fn test_set() {
// let mut a = Vector3f::new(0f32, 0f32, 0f32);
// let b = Vector3f::new(1f32, 2f32, 3f32);
//
// a.set(&b);
//
// assert_eq!(a.x, 1f32);
// assert_eq!(a.y, 2f32);
// assert_eq!(a.z, 3f32);
// }
//
// #[test]
// fn test_sub() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(0f32, 1f32, 2f32);
//
// a.sub(&b);
//
// assert_eq!(a.x, 1f32);
// assert_eq!(a.y, 1f32);
// assert_eq!(a.z, 1f32);
// }
//
// #[test]
// fn test_sub_into() {
// let a = Vector3f::new(1f32, 2f32, 3f32);
// let b = Vector3f::new(0f32, 1f32, 2f32);
// let mut c = Vector3f::new(0f32, 0f32, 0f32);
//
// a.sub_into(&b, &mut c);
//
// assert_eq!(c.x, 1f32);
// assert_eq!(c.y, 1f32);
// assert_eq!(c.z, 1f32);
// }
//
// #[test]
// fn test_zero() {
// let mut a = Vector3f::new(1f32, 2f32, 3f32);
//
// a.zero();
//
// assert_eq!(a.x, 0f32);
// assert_eq!(a.y, 0f32);
// assert_eq!(a.z, 0f32);
// }
// }
//
