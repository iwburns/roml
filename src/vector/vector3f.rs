//use super::Vector;
//use super::Vector3;
//
//pub struct Vector3f {
//    pub x: f32,
//    pub y: f32,
//    pub z: f32,
//}
//
//impl Vector<f32> for Vector3f {
//    fn add(&mut self, v: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn add_into(&self, v: &Vector3f, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn angle(&self, v: &Vector3f) -> f32 {
//        unimplemented!()
//    }
//
//    fn angle_cos(&self, v: &Vector3f) -> f32 {
//        unimplemented!()
//    }
//
//    fn distance(&self, v: &Vector3f) -> f32 {
//        unimplemented!()
//    }
//
//    fn distance_squared(&self, v: &Vector3f) -> f32 {
//        unimplemented!()
//    }
//
//    fn dot(&self, v: &Vector3f) -> f32 {
//        unimplemented!()
//    }
//
//    fn fma_scalar(&mut self, a: f32, b: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn fma_scalar_into(&self, a: f32, b: &Vector3f, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn fma_vector(&mut self, a: &Vector3f, b: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn fma_vector_into(&self, a: &Vector3f, b: &Vector3f, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn length(&self) -> f32 {
//        unimplemented!()
//    }
//
//    fn length_squared(&self) -> f32 {
//        unimplemented!()
//    }
//
//    fn lerp(&mut self, other: &Vector3f, t: f32) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn lerp_into(&self, other: &Vector3f, t: f32, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn mul_scalar(&mut self, s: f32) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn mul_scalar_into(&self, s: f32, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn mul_vector(&mut self, v: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn mul_vector_into(&self, v: &Vector3f, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn negate(&mut self) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn negate_into(&self, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn normalize(&mut self) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn normalize_into(&self, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn set(&mut self, v: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn sub(&mut self, v: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn sub_into(&self, v: &Vector3f, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn zero(&mut self) -> &mut Vector3f {
//        unimplemented!()
//    }
//}
//
//impl Vector3<f32> for Vector3f {
//    fn add_components(&mut self, x: f32, y: f32, z: f32) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn add_components_into(&self, x: f32, y: f32, z: f32, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//
//    fn cross(&mut self, v: &Vector3f) -> &mut Vector3f {
//        unimplemented!()
//    }
//
//    fn cross_into(&self, v: &Vector3f, dest: &mut Vector3f) {
//        unimplemented!()
//    }
//}
