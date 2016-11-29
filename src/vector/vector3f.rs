use super::Vector;
use super::Vector3;
use super::ThreeTuple;

pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector<Vector3f, f32> for Vector3f {
    fn add(&mut self, v: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn add_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn angle(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn distance(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn distance_squared(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn dot(&self, v: &Vector3f) -> f32 {
        unimplemented!()
    }

    fn component_fma(&mut self, a: f32, b: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn component_fma_into(&self, a: f32, b: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn vector_fma(&mut self, a: Vector3f, b: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn vector_fma_into(&self, a: Vector3f, b: &Vector3f, dest: &mut Vector3f) {
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

    fn lerp_into(&self, t: f32, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn scalar_mul(&mut self, s: f32) -> &mut Self {
        unimplemented!()
    }

    fn scalar_mul_into(&self, s: f32, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn vector_mul(&mut self, v: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn vector_mul_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn negate(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn negate_into(&self, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn normalize(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn normalize_into(&self, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn set(&mut self, v: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn sub(&mut self, v: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn sub_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn zero(&mut self) -> &mut Self {
        unimplemented!()
    }
}

impl Vector3<Vector3f, f32> for Vector3f {
    fn add_components(&mut self, tuple: ThreeTuple<f32>) -> &mut Self {
        unimplemented!()
    }

    fn add_components_into(&self, tuple: ThreeTuple<f32>, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn cross(&mut self, v: &Vector3f) -> &mut Self {
        unimplemented!()
    }

    fn cross_into(&self, v: &Vector3f, dest: &mut Vector3f) {
        unimplemented!()
    }
}