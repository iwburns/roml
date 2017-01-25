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

    fn get_column(&self, column: Self::ColIndex) -> Self::ColVector;
    fn copy_column(&self, column: Self::ColIndex, dest: &mut Self::ColVector);
    fn set_column(&mut self, column: Self::ColIndex, source: &Self::ColVector);

    fn get_row(&self, row: Self::RowIndex) -> Self::RowVector;
    fn copy_row(&self, row: Self::RowIndex, dest: &mut Self::RowVector);
    fn set_row(&mut self, row: Self::RowIndex, source: &Self::RowVector);

    fn get_cell(&self, column: Self::ColIndex, row: Self::RowIndex) -> &T;
    fn copy_cell(&self, column: Self::ColIndex, row: Self::RowIndex, dest: &mut T);
    fn set_cell(&mut self, column: Self::ColIndex, row: Self::RowIndex, source: &T);

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
    Zero,
    One,
}

pub enum Mat3Index {
    Zero,
    One,
    Two,
}

pub enum Mat4Index {
    Zero,
    One,
    Two,
    Three,
}

pub mod matrix3f;
