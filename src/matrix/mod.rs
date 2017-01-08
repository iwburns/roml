use super::vector::Vector3;

// Self just needs to be a matrix.  Size and shape of Self do not matter.
pub trait Matrix<T> {
    fn add(&mut self, m: &Self) -> &mut Self;
    fn add_into(&self, m: &Self, dest: &mut Self);
    fn sub(&mut self, m: &Self) -> &mut Self;
    fn sub_into(&self, m: &Self, dest: &mut Self);
    // todo: add more here
}

// Self must be a Square Matrix, but these functions cannot depend upon the size of Self.
pub trait SquareMatrix<T>: Matrix<T> {
    fn determinant(&self) -> T;
    fn identity(&mut self) -> &mut Self;
    fn invert(&mut self) -> &mut Self;
    fn invert_into(&self, dest: &mut Self);
    // todo: add more here
}

pub enum Mat3Index {
    One,
    Two,
    Three,
}

// Self has to be a square 3x3 matrix
pub trait Matrix3<T>: SquareMatrix<T> {
    type V: Vector3<T>;

    fn get_column(&self, column: Mat3Index, dest: &mut Self::V);
    fn get_row(&self, row: Mat3Index, dest: &mut Self::V);
}

pub mod matrix3f;
