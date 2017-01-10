use matrix::*;
use vector::vector3f::Vector3f;

pub struct Matrix3f {}

impl Matrix<f32> for Matrix3f {
    type RowIndex = Mat3Index;
    type ColIndex = Mat3Index;
    type RowVector = Vector3f;
    type ColVector = Vector3f;

    fn add(&mut self, m: &Self) -> &mut Self {
        unimplemented!()
    }

    fn add_into(&self, m: &Self, dest: &mut Self) {
        unimplemented!()
    }

    fn sub(&mut self, m: &Self) -> &mut Self {
        unimplemented!()
    }

    fn sub_into(&self, m: &Self, dest: &mut Self) {
        unimplemented!()
    }

    fn get_column(&self, column: Mat3Index, dest: &mut Vector3f) {
        unimplemented!()
    }

    fn get_row(&self, column: Mat3Index, dest: &mut Vector3f) {
        unimplemented!()
    }
}

impl SquareMatrix<f32> for Matrix3f {
    fn determinant(&self) -> f32 {
        unimplemented!()
    }

    fn identity(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn invert(&mut self) -> &mut Self {
        unimplemented!()
    }

    fn invert_into(&self, dest: &mut Self) {
        unimplemented!()
    }
}
