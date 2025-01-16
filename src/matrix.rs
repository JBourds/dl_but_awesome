pub use internal::{Matrix, Shape};

#[allow(dead_code)]
pub mod internal {
    pub trait Numeric {}
    impl Numeric for f32 {}

    #[derive(Debug, PartialEq)]
    pub struct Shape(Vec<usize>);

    impl Shape {
        pub fn new(dimensions: Vec<usize>) -> Self {
            Self(dimensions)
        }
        pub fn len(&self) -> usize {
            self.0.iter().product()
        }
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Matrix<T> {
        data: Vec<T>,
        shape: Shape,
    }

    impl<T: Numeric + Copy> Matrix<T> {
        pub fn new_uninit(shape: Shape) -> Self {
            Self {
                data: Vec::with_capacity(shape.len()),
                shape,
            }
        }
        pub fn default(value: T, shape: Shape) -> Self {
            Self {
                data: vec![value; shape.len()],
                shape,
            }
        }
    }

    impl Matrix<f32> {
        pub fn zeros(shape: Shape) -> Self {
            Matrix::<f32>::default(0.0, shape)
        }
        pub fn ones(shape: Shape) -> Self {
            Matrix::<f32>::default(1.0, shape)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matrix() {
        let shape = Shape::new(vec![5, 5]);
        let m = Matrix::<f32>::new_uninit(shape);
        println!("{:#?}", m);
        let shape = Shape::new(vec![5, 5]);
        let m = Matrix::<f32>::zeros(shape);
        println!("{:#?}", m);
    }
}
