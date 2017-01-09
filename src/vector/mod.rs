// todo:
// for _into methods, add two more generic types `D` and `T`.
// `D` will be specific to just those functions
// `T` will be generic across the whole Vector<C>
//
// change to: pub trait Vector<C, T> {}
// and it will be impl'd like this:
// impl Vector<f32, Vector2> for Vector2f {}
//
// then add getters/setters to Vector2
// (if needed) make Vector2 not a Sub-Trait of Vector
//
// then the trait bound on `D` can be `where D: T`
// this allows us to just call setters on D no matter what type it is.
// need to evaluate performance of this concept.
//


// todo: change zero to clear or something like that
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
    fn length_sq(&self) -> C;
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

pub trait Vector3<C>: Vector<C> {
    fn new(x: C, y: C, z: C) -> Self;
    fn cross<'a, V: 'a>(&mut self, v: &'a V) -> &mut Self where Self: From<&'a V>;
    fn cross_into<'a, V: 'a>(&self, v: &'a V, dest: &mut Self) where Self: From<&'a V>;
}

pub trait Vector4<C>: Vector<C> {
    fn new(x: C, y: C, z: C, w: C) -> Self;
}

pub mod vector2f;
pub mod vector3f;
pub mod vector4f;
