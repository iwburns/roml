//todo: possibly get rid of VecType here and just use Self

pub trait Vector<VecType, CompType> {
    fn new() -> Self;
    fn add(&mut self, v: &VecType) -> &mut Self;
    fn add_into(&self, v: &VecType, dest: &mut VecType);
    fn angle(&self, v: &VecType) -> CompType;
    fn angle_cos(&self, v: &VecType) -> CompType;
    fn distance(&self, v: &VecType) -> CompType;
    fn distance_squared(&self, v: &VecType) -> CompType;
    fn dot(&self, v: &VecType) -> CompType;
    fn fma_scalar(&mut self, a: CompType, b: &VecType) -> &mut Self;
    fn fma_scalar_into(&self, a: CompType, b: &VecType, dest: &mut VecType);
    fn fma_vector(&mut self, a: &VecType, b: &VecType) -> &mut Self;
    fn fma_vector_into(&self, a: &VecType, b: &VecType, dest: &mut VecType);
    fn length(&self) -> CompType;
    fn length_squared(&self) -> CompType;
    fn lerp(&mut self, other: &VecType, t: CompType) -> &mut Self;
    fn lerp_into(&self, other: &VecType, t: CompType, dest: &mut VecType);
    fn mul_scalar(&mut self, s: CompType) -> &mut Self;
    fn mul_scalar_into(&self, s: CompType, dest: &mut VecType);
    fn mul_vector(&mut self, v: &VecType) -> &mut Self;
    fn mul_vector_into(&self, v: &VecType, dest: &mut VecType);
    fn negate(&mut self) -> &mut Self;
    fn negate_into(&self, dest: &mut VecType);
    fn normalize(&mut self) -> &mut Self;
    fn normalize_into(&self, dest: &mut VecType);
    fn set(&mut self, v: &VecType) -> &mut Self;
    fn sub(&mut self, v: &VecType) -> &mut Self;
    fn sub_into(&self, v: &VecType, dest: &mut VecType);
    fn zero(&mut self) -> &mut Self;
}

pub trait Vector2<VecType, CompType> : Vector<VecType, CompType> {
    fn add_components(&mut self, x: CompType, y: CompType) -> &mut Self;
    fn add_components_into(&self, x: CompType, y: CompType, dest: &mut VecType);
}

pub trait Vector3<VecType, CompType> : Vector<VecType, CompType> {
    fn add_components(&mut self, x: CompType, y: CompType, z: CompType) -> &mut Self;
    fn add_components_into(&self, x: CompType, y: CompType, z: CompType, dest: &mut VecType);
    fn cross(&mut self, v: &VecType) -> &mut Self;
    fn cross_into(&self, v: &VecType, dest: &mut VecType);
}

pub trait Vector4<VecType, CompType> : Vector<VecType, CompType> {
    fn add_components(&mut self, x: CompType, y: CompType, z: CompType, w: CompType) -> &mut Self;
    fn add_components_into(&self, x: CompType, y: CompType, z: CompType, w: CompType, dest: &mut VecType);
}

pub mod vector2f;
pub mod vector3f;
pub mod vector4f;
