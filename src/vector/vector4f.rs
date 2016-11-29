use super::Vector;
use super::Vector4;
use super::FourTuple;

pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector<Vector4f, f32> for Vector4f {
    fn add(&mut self, v: &Vector4f) -> &mut Self {
        unimplemented!()
    }

    fn add_into(&self, v: &Vector4f, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn angle(&self, v: &Vector4f) -> f32 {
        unimplemented!()
    }

    fn distance(&self, v: &Vector4f) -> f32 {
        unimplemented!()
    }

    fn distance_squared(&self, v: &Vector4f) -> f32 {
        unimplemented!()
    }

    fn dot(&self, v: &Vector4f) -> f32 {
        unimplemented!()
    }

    fn component_fma(&mut self, a: f32, b: &Vector4f) -> &mut Self {
        unimplemented!()
    }

    fn component_fma_into(&self, a: f32, b: &Vector4f, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn vector_fma(&mut self, a: Vector4f, b: &Vector4f) -> &mut Self {
        unimplemented!()
    }

    fn vector_fma_into(&self, a: Vector4f, b: &Vector4f, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn length(&self) -> f32 {
        unimplemented!()
    }

    fn length_squared(&self) -> f32 {
        unimplemented!()
    }

    fn lerp(&mut self, t: f32) -> &mut Self {
        unimplemented!()
    }

    fn lerp_into(&self, t: f32, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn scalar_mul(&mut self, s: f32) -> &mut Self {
        unimplemented!()
    }

    fn scalar_mul_into(&self, s: f32, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn vector_mul(&mut self, v: &Vector4f) -> &mut Self {
        unimplemented!()
    }

    fn vector_mul_into(&self, v: &Vector4f, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn negate(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn negate_into(&self, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn normalize(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn normalize_into(&self, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn set(&mut self, v: &Vector4f) -> &mut Self {
        unimplemented!()
    }

    fn sub(&mut self, v: &Vector4f) -> &mut Self {
        unimplemented!()
    }

    fn sub_into(&self, v: &Vector4f, dest: &mut Vector4f) {
        unimplemented!()
    }

    fn zero(&mut self) -> &mut Self {
        unimplemented!()
    }
}

impl Vector4<Vector4f, f32> for Vector4f {
    fn add_components(&mut self, tuple: FourTuple<f32>) -> &mut Self {
        unimplemented!()
    }

    fn add_components_into(&self, tuple: FourTuple<f32>, dest: &mut Vector4f) {
        unimplemented!()
    }
}
