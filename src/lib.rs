#![allow(unused_variables)]

pub type TwoTuple<T> = (T, T);
pub type ThreeTuple<T> = (T, T, T);
pub type FourTuple<T> = (T, T, T, T);

pub mod vector;
//pub mod matrix;

//pub use vector::Vector;
pub use vector::vector2f::Vector2f;
