pub type TwoTuple<T> = (T, T);
pub type ThreeTuple<T> = (T, T, T);
pub type FourTuple<T> = (T, T, T, T);

pub trait Vector<VectorType, ComponentType> {
    fn add(&mut self, v: &VectorType) -> &mut Self;
    fn add_into(&self, v: &VectorType, dest: &mut VectorType);
    fn angle(&self, v: &VectorType) -> ComponentType;
    fn distance(&self, v: &VectorType) -> ComponentType;
    fn distance_squared(&self, v: &VectorType) -> ComponentType;
    fn dot(&self, v: &VectorType) -> ComponentType;
    fn component_fma(&mut self, a: ComponentType, b: &VectorType) -> &mut Self;
    fn component_fma_into(&self, a: ComponentType, b: &VectorType, dest: &mut VectorType);
    fn vector_fma(&mut self, a: VectorType, b: &VectorType) -> &mut Self;
    fn vector_fma_into(&self, a: VectorType, b: &VectorType, dest: &mut VectorType);
    fn length(&self) -> ComponentType;
    fn length_squared(&self) -> ComponentType;
    fn lerp(&mut self, t: ComponentType) -> &mut Self;
    fn lerp_into(&self, t: ComponentType, dest: &mut VectorType);
    fn scalar_mul(&mut self, s: ComponentType) -> &mut Self;
    fn scalar_mul_into(&self, s: ComponentType, dest: &mut VectorType);
    fn vector_mul(&mut self, v: &VectorType) -> &mut Self;
    fn vector_mul_into(&self, v: &VectorType, dest: &mut VectorType);
    fn negate(&mut self) -> &mut Self;
    fn negate_into(&self, dest: &mut VectorType);
    fn normalize(&mut self) -> &mut Self;
    fn normalize_into(&self, dest: &mut VectorType);
    fn set(&mut self, v: &VectorType) -> &mut Self;
    fn sub(&mut self, v: &VectorType) -> &mut Self;
    fn sub_into(&self, v: &VectorType, dest: &mut VectorType);
    fn zero(&mut self) -> &mut Self;
}

pub trait Vector2<VectorType, ComponentType> : Vector<VectorType, ComponentType> {
    fn add_components(&mut self, tuple: TwoTuple<ComponentType>) -> &mut Self;
    fn add_components_into(&self, tuple: TwoTuple<ComponentType>, dest: &mut VectorType);
}

pub trait Vector3<VectorType, ComponentType> : Vector<VectorType, ComponentType> {
    fn add_components(&mut self, tuple: ThreeTuple<ComponentType>) -> &mut Self;
    fn add_components_into(&self, tuple: ThreeTuple<ComponentType>, dest: &mut VectorType);
    fn cross(&mut self, v: &VectorType) -> &mut Self;
    fn cross_into(&self, v: &VectorType, dest: &mut VectorType);
}

pub trait Vector4<VectorType, ComponentType> : Vector<VectorType, ComponentType> {
    fn add_components(&mut self, tuple: FourTuple<ComponentType>) -> &mut Self;
    fn add_components_into(&self, tuple: FourTuple<ComponentType>, dest: &mut VectorType);
}

pub mod vector2f;
pub mod vector3f;
