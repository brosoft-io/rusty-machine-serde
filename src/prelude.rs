//! The rusty-machine prelude.
//!
//! This module alleviates some common imports used within rusty-machine.

pub use linalg_serde::{Matrix, MatrixSlice, MatrixSliceMut, BaseMatrix, BaseMatrixMut};
pub use linalg_serde::Vector;
pub use linalg_serde::Axes;

pub use learning::SupModel;
pub use learning::UnSupModel;

#[cfg(test)]
mod tests {
    use super::super::prelude::*;

    #[test]
    fn create_mat_from_prelude() {
        let _ = Matrix::new(2, 2, vec![4.0;4]);
    }
}
