// Self just needs to be a matrix.  Size and shape of Self do not matter.
pub trait Matrix<T> {
    type RowIndex;
    type ColIndex;
    type RowVector;
    type ColVector;

    fn add(&mut self, m: &Self) -> &mut Self;
    fn add_into(&self, m: &Self, dest: &mut Self);
    fn sub(&mut self, m: &Self) -> &mut Self;
    fn sub_into(&self, m: &Self, dest: &mut Self);
    fn get_column(&self, column: Self::ColIndex, dest: &mut Self::ColVector);
    fn get_row(&self, column: Self::RowIndex, dest: &mut Self::RowVector);
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

pub enum Mat2Index {
    One,
    Two,
}

pub enum Mat3Index {
    One,
    Two,
    Three,
}

pub enum Mat4Index {
    One,
    Two,
    Three,
}

pub mod matrix3f;
