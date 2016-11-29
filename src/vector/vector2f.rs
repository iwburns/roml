use super::Vector;
use super::Vector2;

pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector<Vector2f, f32> for Vector2f {
    fn add(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn add_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn angle(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn distance(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn distance_squared(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn dot(&self, v: &Vector2f) -> f32 {
        unimplemented!()
    }

    fn component_fma(&mut self, a: f32, b: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn component_fma_into(&self, a: f32, b: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn vector_fma(&mut self, a: Vector2f, b: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn vector_fma_into(&self, a: Vector2f, b: &Vector2f, dest: &mut Vector2f) {
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

    fn lerp_into(&self, t: f32, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn scalar_mul(&mut self, s: f32) -> &mut Self {
        unimplemented!()
    }

    fn scalar_mul_into(&self, s: f32, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn vector_mul(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn vector_mul_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn negate(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn negate_into(&self, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn normalize(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn normalize_into(&self, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn set(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn sub(&mut self, v: &Vector2f) -> &mut Self {
        unimplemented!()
    }

    fn sub_into(&self, v: &Vector2f, dest: &mut Vector2f) {
        unimplemented!()
    }

    fn zero(&mut self) -> &mut Self {
        unimplemented!()
    }
}

impl Vector2<Vector2f, f32> for Vector2f {
    fn add_components(&mut self, x: f32, y: f32) -> &mut Self {
        unimplemented!()
    }

    fn add_components_into(&self, x: f32, y: f32, dest: &mut Vector2f) {
        unimplemented!()
    }
}
