use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default)]
pub struct Tensor<T>
{
    pub(crate) data: Vec<T>,
    pub(crate) dimensions: Vec<usize>,
}

impl<T: Default + Clone> Tensor<T>
{
    pub fn new(dimensions: Vec<usize>) -> Self
    {
        let size = dimensions.iter().product();
        Self {
            data: vec![T::default(); size],
            dimensions,
        }
    }

    pub fn from_vec(data: Vec<T>, dimensions: Vec<usize>) -> Self
    {
        assert_eq!(data.len(), dimensions.iter().product::<usize>());
        Self { data, dimensions }
    }

    pub fn dimensions(&self) -> &[usize]
    {
        &self.dimensions
    }

    pub fn index_to_flat_index(&self, indices: &[usize]) -> usize
    {
        assert_eq!(indices.len(), self.dimensions.len());
        indices
            .iter()
            .zip(&self.dimensions)
            .fold(0, |acc, (&idx, &dim)| {
                assert!(idx < dim);
                acc * dim + idx
            })
    }

    pub fn add_dimension(&mut self, dimension_size: usize)
    {
        self.dimensions.push(dimension_size);
    }

    pub fn add_value(&mut self, indices: &[usize], value: T){
        assert!(self.dimensions.len() - 1 == indices.len(), "The number of indeces must be 'dimensions.len() - 1'");
        let vec = self.data.index_mut(indices);
        
    }





























}

impl<T: Default + Clone> Index<&[usize]> for Tensor<T>
{
    type Output = T;

    fn index(&self, indices: &[usize]) -> &Self::Output
    {
        let flat_index = self.index_to_flat_index(indices);
        &self.data[flat_index]
    }
}

impl<T: Default + Clone> IndexMut<&[usize]> for Tensor<T>
{
    fn index_mut(&mut self, indices: &[usize]) -> &mut Self::Output
    {
        let flat_index = self.index_to_flat_index(indices);
        &mut self.data[flat_index]
    }
}
