pub trait Vector<C>
    where Self: Sized
{
    fn add<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn add_into<'a, V: 'a>(&self, rhs: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn angle<'a, V: 'a>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn angle_cos<'a, V: 'a>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn distance<'a, V: 'a>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn distance_sq<'a, V: 'a>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn dot<'a, V: 'a>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn fma<'a, V: 'a>(&mut self, a: &'a V, b: &'a V) -> &mut Self where Self: From<&'a V>;
    fn fma_into<'a, V: 'a>(&self, a: &'a V, b: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn length(&self) -> C;
    fn length_squared(&self) -> C;
    fn lerp<'a, V: 'a>(&mut self, other: &'a V, t: C) -> &mut Self where Self: From<&'a V>;
    fn lerp_into<'a, V: 'a>(&self, other: &'a V, t: C, dest: &mut Self) where Self: From<&'a V>;
    fn mul<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn mul_into<'a, V: 'a>(&self, rhs: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn negate(&mut self) -> &mut Self;
    fn negate_into(&self, dest: &mut Self);
    fn normalize(&mut self) -> &mut Self;
    fn normalize_into(&self, dest: &mut Self);
    fn set<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn sub<'a, V: 'a>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn sub_into<'a, V: 'a>(&self, rhs: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn zero(&mut self) -> &mut Self;
}

pub trait Vector2<C>: Vector<C> {
    fn new(x: C, y: C) -> Self;
}

// pub trait Vector3<T> : Vector<T> {
//    fn add_components(&mut self, x: T, y: T, z: T) -> &mut Self;
//    fn add_components_into(&self, x: T, y: T, z: T, dest: &mut Self);
//    fn cross(&mut self, v: &Self) -> &mut Self;
//    fn cross_into(&self, v: &Self, dest: &mut Self);
//
//
// pub trait Vector4<T> : Vector<T> {
//    fn add_components(&mut self, x: T, y: T, z: T, w: T) -> &mut Self;
//    fn add_components_into(&self, x: T, y: T, z: T, w: T, dest: &mut Self);
//

pub mod vector2f;
// pub mod vector3f;
// pub mod vector4f;
