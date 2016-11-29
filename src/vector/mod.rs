pub type TwoTuple<T> = (T, T);
pub type ThreeTuple<T> = (T, T, T);
pub type FourTuple<T> = (T, T, T, T);

pub trait Vector<VecType, CompType> {
    fn add(&mut self, v: &VecType) -> &mut Self;
    fn add_into(&self, v: &VecType, dest: &mut VecType);
    fn angle(&self, v: &VecType) -> CompType;
    fn distance(&self, v: &VecType) -> CompType;
    fn distance_squared(&self, v: &VecType) -> CompType;
    fn dot(&self, v: &VecType) -> CompType;
    fn component_fma(&mut self, a: CompType, b: &VecType) -> &mut Self;
    fn component_fma_into(&self, a: CompType, b: &VecType, dest: &mut VecType);
    fn vector_fma(&mut self, a: VecType, b: &VecType) -> &mut Self;
    fn vector_fma_into(&self, a: VecType, b: &VecType, dest: &mut VecType);
    fn length(&self) -> CompType;
    fn length_squared(&self) -> CompType;
    fn lerp(&mut self, t: CompType) -> &mut Self;
    fn lerp_into(&self, t: CompType, dest: &mut VecType);
    fn scalar_mul(&mut self, s: CompType) -> &mut Self;
    fn scalar_mul_into(&self, s: CompType, dest: &mut VecType);
    fn vector_mul(&mut self, v: &VecType) -> &mut Self;
    fn vector_mul_into(&self, v: &VecType, dest: &mut VecType);
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
    fn add_components(&mut self, tuple: TwoTuple<CompType>) -> &mut Self;
    fn add_components_into(&self, tuple: TwoTuple<CompType>, dest: &mut VecType);
}

pub trait Vector3<VecType, CompType> : Vector<VecType, CompType> {
    fn add_components(&mut self, tuple: ThreeTuple<CompType>) -> &mut Self;
    fn add_components_into(&self, tuple: ThreeTuple<CompType>, dest: &mut VecType);
    fn cross(&mut self, v: &VecType) -> &mut Self;
    fn cross_into(&self, v: &VecType, dest: &mut VecType);
}

pub trait Vector4<VecType, CompType> : Vector<VecType, CompType> {
    fn add_components(&mut self, tuple: FourTuple<CompType>) -> &mut Self;
    fn add_components_into(&self, tuple: FourTuple<CompType>, dest: &mut VecType);
}

pub mod vector2f;
pub mod vector3f;
pub mod vector4f;
