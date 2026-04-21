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

    fn modify<O>(
        &mut self,
        address: T,
        modifier: impl FnOnce(&mut T) -> O,
    ) -> Result<O, Self::Error> {
        Ok(modifier(self.0.entry(address).or_default()))
    }
}
