pub trait Numeric {}
impl Numeric for f32 {}

#[derive(Debug, PartialEq)]
struct Shape(Vec<usize>);

impl Shape {
    pub fn len(&self) -> usize {
        self.0.iter().product()
    }
}

#[derive(Debug, PartialEq)]
struct Matrix<T> {
    data: Vec<T>,
    shape: Shape,
}

impl<T: Numeric + Copy> Matrix<T> {
    fn new_uninit(shape: Shape) -> Self {
        Self {
            data: Vec::with_capacity(shape.len()),
            shape,
        }
    }
    fn default(value: T, shape: Shape) -> Self {
        Self {
            data: vec![value; shape.len()],
            shape,
        }
    }
}

impl Matrix<f32> {
    fn zeros(shape: Shape) -> Self {
        Matrix::<f32>::default(0.0, shape)
    }
    fn ones(shape: Shape) -> Self {
        Matrix::<f32>::default(1.0, shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matrix() {
        let shape = Shape(vec![5, 5]);
        let mut m = Matrix::<f32>::new_uninit(shape);
        println!("{:#?}", m);
        let shape = Shape(vec![5, 5]);
        let mut m = Matrix::<f32>::zeroed(shape);
        println!("{:#?}", m);
    }
}
