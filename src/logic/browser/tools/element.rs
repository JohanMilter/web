use crate::logic::browser::drivers::behaviors::{DriverRead, DriverWrite};

use super::behaviors::{ElementRead, ElementWrite};

pub struct Element<D: DriverRead + DriverWrite>{
    state: std::marker::PhantomData<D>,
}
impl<D: DriverRead + DriverWrite> ElementRead for Element<D> {
    
}
impl<D: DriverRead + DriverWrite> ElementWrite for Element<D> {
    
}