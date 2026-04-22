use core::{convert::Infallible, hash::Hash};

use std::collections::HashMap;

use super::Memory;

#[derive(Clone, Debug, Default)]
pub struct HashMapMemory<T>(pub HashMap<T, T>);

impl<T> Memory<T> for HashMapMemory<T>
where
    T: Hash + Ord + Clone + Default,
{
    type Error = Infallible;

    fn get(&self, address: &T) -> Result<T, Self::Error> {
        Ok(self.0.get(address).cloned().unwrap_or(T::default()))
    }

    fn set(&mut self, address: T, value: T) -> Result<(), Self::Error> {
        self.0.insert(address, value);
        Ok(())
    }
}
