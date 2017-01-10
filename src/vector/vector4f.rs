use super::Vector;
use super::Vector4;

pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector<f32> for Vector4f {
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

    fn length_sq(&self) -> f32 {
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

impl Vector4<f32> for Vector4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        unimplemented!()
    }
}
