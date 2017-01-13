
// todo: change zero to clear or something like that
//pub trait Vector<C>
//    where Self: Sized
//{
//
//}

pub trait Vector2<C> {
    fn new(x: C, y: C) -> Self;
    fn set_x(&mut self, x: C);
    fn set_y(&mut self, y: C);
    fn add<V>(&mut self, rhs: &V) -> &mut Self where V: ReadVector2<C>;
    fn add_into<V>(&self, rhs: &V, dest: &mut Self) where V: ReadVector2<C>;
    fn angle<V>(&self, rhs: &V) -> C where V: ReadVector2<C>;
    fn angle_cos<V>(&self, rhs: &V) -> C where V: ReadVector2<C>;
    fn distance<V>(&self, rhs: &V) -> C where V: ReadVector2<C>;
    fn distance_sq<V>(&self, rhs: &V) -> C where V: ReadVector2<C>;
    fn dot<V>(&self, rhs: &V) -> C where V: ReadVector2<C>;
    fn fma<V>(&mut self, a: &V, b: &V) -> &mut Self where V: ReadVector2<C>;
    fn fma_into<V>(&self, a: &V, b: &V, dest: &mut Self) where V: ReadVector2<C>;
    fn length(&self) -> C;
    fn length_sq(&self) -> C;
    fn lerp<V>(&mut self, other: &V, t: C) -> &mut Self where V: ReadVector2<C>;
    fn lerp_into<V>(&self, other: &V, t: C, dest: &mut Self) where V: ReadVector2<C>;
    fn mul<V>(&mut self, rhs: &V) -> &mut Self where V: ReadVector2<C>;
    fn mul_into<V>(&self, rhs: &V, dest: &mut Self) where V: ReadVector2<C>;
    fn negate(&mut self) -> &mut Self;
    fn negate_into(&self, dest: &mut Self);
    fn normalize(&mut self) -> &mut Self;
    fn normalize_into(&self, dest: &mut Self);
    fn set<V>(&mut self, rhs: &V) -> &mut Self where V: ReadVector2<C>;
    fn sub<V>(&mut self, rhs: &V) -> &mut Self where V: ReadVector2<C>;
    fn sub_into<V>(&self, rhs: &V, dest: &mut Self) where V: ReadVector2<C>;
    fn zero(&mut self) -> &mut Self;
}

pub trait ReadVector2<C> {
    fn x(&self) -> C;
    fn y(&self) -> C;
}

//pub trait Vector3<C>: Vector<C> {
//    fn new(x: C, y: C, z: C) -> Self;
//    fn cross<V>(&mut self, v: &V) -> &mut Self where V: ReadVector3<C>;
//    fn cross_into<V>(&self, v: &V, dest: &mut Self) where V: ReadVector3<C>;
//}
//
//pub trait Vector4<C>: Vector<C> {
//    fn new(x: C, y: C, z: C, w: C) -> Self;
//}

pub mod vector2f;
//pub mod vector3f;
//pub mod vector4f;
