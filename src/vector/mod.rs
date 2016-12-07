use std;

pub type TwoTuple<T> = (T, T);

pub trait Vector<T> where Self: std::marker::Sized {
    fn add<'a, V>(&mut self, v: &'a V) -> &mut Self where &'a V: Into<Self>;
//    fn add_into(&self, v: &Self, dest: &mut Self);
//    fn angle(&self, v: &Self) -> T;
//    fn angle_cos(&self, v: &Self) -> T;
//    fn distance(&self, v: &Self) -> T;
//    fn distance_squared(&self, v: &Self) -> T;
//    fn dot(&self, v: &Self) -> T;
//    fn fma_scalar(&mut self, a: T, b: &Self) -> &mut Self;
//    fn fma_scalar_into(&self, a: T, b: &Self, dest: &mut Self);
//    fn fma_vector(&mut self, a: &Self, b: &Self) -> &mut Self;
//    fn fma_vector_into(&self, a: &Self, b: &Self, dest: &mut Self);
//    fn length(&self) -> T;
//    fn length_squared(&self) -> T;
//    fn lerp(&mut self, other: &Self, t: T) -> &mut Self;
//    fn lerp_into(&self, other: &Self, t: T, dest: &mut Self);
//    fn mul_scalar(&mut self, s: T) -> &mut Self;
//    fn mul_scalar_into(&self, s: T, dest: &mut Self);
//    fn mul_vector(&mut self, v: &Self) -> &mut Self;
//    fn mul_vector_into(&self, v: &Self, dest: &mut Self);
//    fn negate(&mut self) -> &mut Self;
//    fn negate_into(&self, dest: &mut Self);
//    fn normalize(&mut self) -> &mut Self;
//    fn normalize_into(&self, dest: &mut Self);
//    fn set(&mut self, v: &Self) -> &mut Self;
//    fn sub(&mut self, v: &Self) -> &mut Self;
//    fn sub_into(&self, v: &Self, dest: &mut Self);
//    fn zero(&mut self) -> &mut Self;
}

pub trait Vector2<T> : Vector<T> {
    fn new(x: T, y: T) -> Self;
//    fn add_components(&mut self, x: T, y: T) -> &mut Self;
//    fn add_components_into(&self, x: T, y: T, dest: &mut Self);
}

//pub trait Vector3<T> : Vector<T> {
//    fn add_components(&mut self, x: T, y: T, z: T) -> &mut Self;
//    fn add_components_into(&self, x: T, y: T, z: T, dest: &mut Self);
//    fn cross(&mut self, v: &Self) -> &mut Self;
//    fn cross_into(&self, v: &Self, dest: &mut Self);
//}
//
//pub trait Vector4<T> : Vector<T> {
//    fn add_components(&mut self, x: T, y: T, z: T, w: T) -> &mut Self;
//    fn add_components_into(&self, x: T, y: T, z: T, w: T, dest: &mut Self);
//}

pub mod vector2f;
//pub mod vector3f;
//pub mod vector4f;
