use core::{convert::Infallible, hash::Hash};

use std::collections::HashMap;

use nostd_cow::RefCow;

use super::Memory;

#[derive(Clone, Debug, Default)]
pub struct HashMapMemory<T>(pub HashMap<T, T>);

impl<T> Memory<T> for HashMapMemory<T>
where
    T: Hash + Ord + Clone + Default,
{
    type Error = Infallible;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error> {
        Ok(self
            .0
            .get(&address)
            .map(RefCow::Borrowed)
            .unwrap_or_else(|| RefCow::Owned(T::default())))
    }

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error> {
        self.0.insert(address.into_owned(), value.into_owned());
        Ok(())
    }
}
