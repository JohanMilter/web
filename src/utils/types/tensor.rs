use std::{fmt, ops::{Index, IndexMut}};

#[derive(Debug, Clone, Default)]
pub struct Tensor<T>
{
    pub(crate) data: Vec<T>,
    pub(crate) shape: Vec<usize>,
}
impl<T: fmt::Display> fmt::Display for Tensor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tensor {{")?;
        writeln!(f, "    data: [")?;
        for item in &self.data {
            writeln!(f, "        {},", item)?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "    shape: [")?;
        for dim in &self.shape {
            writeln!(f, "        {},", dim)?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "}}")
    }
}
impl<T: Default + Clone> Tensor<T>
{
    pub fn new(shape: Vec<usize>) -> Self
    {
        let size = shape.iter().product();
        Self {
            data: vec![T::default(); size],
            shape,
        }
    }

    pub fn from_vec(data: Vec<T>, shape: Vec<usize>) -> Self
    {
        assert_eq!(data.len(), shape.iter().product::<usize>());
        Self { data, shape }
    }

    pub fn shape(&self) -> &[usize]
    {
        &self.shape
    }

    pub fn index_to_flat_index(&self, indices: &[usize]) -> usize
    {
        assert_eq!(indices.len(), self.shape.len());
        indices
            .iter()
            .zip(&self.shape)
            .fold(0, |acc, (&idx, &dim)| {
                assert!(idx < dim);
                acc * dim + idx
            })
    }

    pub fn insert_value(&mut self, indices: &[usize], value: T){
        assert_eq!(indices.len(), self.shape.len());
        let flat_index = self.index_to_flat_index(indices);
        self.data.insert(flat_index, value);
    }





























}
