
// todo: change zero to clear or something like that
pub trait Vector<C>
    where Self: Sized
{
    fn add<'a, V>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn add_into<'a, V>(&self, rhs: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn angle<'a, V>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn angle_cos<'a, V>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn distance<'a, V>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn distance_sq<'a, V>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn dot<'a, V>(&self, rhs: &'a V) -> C where Self: From<&'a V>;
    fn fma<'a, V>(&mut self, a: &'a V, b: &'a V) -> &mut Self where Self: From<&'a V>;
    fn fma_into<'a, V>(&self, a: &'a V, b: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn length(&self) -> C;
    fn length_sq(&self) -> C;
    fn lerp<'a, V>(&mut self, other: &'a V, t: C) -> &mut Self where Self: From<&'a V>;
    fn lerp_into<'a, V>(&self, other: &'a V, t: C, dest: &mut Self) where Self: From<&'a V>;
    fn mul<'a, V>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn mul_into<'a, V>(&self, rhs: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn negate(&mut self) -> &mut Self;
    fn negate_into(&self, dest: &mut Self);
    fn normalize(&mut self) -> &mut Self;
    fn normalize_into(&self, dest: &mut Self);
    fn set<'a, V>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn sub<'a, V>(&mut self, rhs: &'a V) -> &mut Self where Self: From<&'a V>;
    fn sub_into<'a, V>(&self, rhs: &'a V, dest: &mut Self) where Self: From<&'a V>;
    fn zero(&mut self) -> &mut Self;
}

pub trait Vector2<C>: Vector<C> {
    fn new(x: C, y: C) -> Self;
}

pub trait Vector3<C>: Vector<C> {
    fn new(x: C, y: C, z: C) -> Self;
    fn cross<'a, V>(&mut self, v: &'a V) -> &mut Self where Self: From<&'a V>;
    fn cross_into<'a, V>(&self, v: &'a V, dest: &mut Self) where Self: From<&'a V>;
}

pub trait Vector4<C>: Vector<C> {
    fn new(x: C, y: C, z: C, w: C) -> Self;
}

pub mod vector2f;
pub mod vector3f;
pub mod vector4f;
