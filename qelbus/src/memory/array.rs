use nostd_cow::RefCow;

use super::Memory;

pub trait ToIndex: Sized {
    fn to_index(address: RefCow<'_, Self>) -> Option<usize>;
}

pub struct AddressOutOfRangeError;

pub struct ArrayMemory<const N: usize, T>([T; N]);

impl<const N: usize, T> Memory<T> for ArrayMemory<N, T>
where
    T: Clone + ToIndex,
{
    type Error = AddressOutOfRangeError;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error> {
        let address = ToIndex::to_index(address).ok_or(AddressOutOfRangeError)?;
        self.0
            .get(address)
            .map(RefCow::Borrowed)
            .ok_or(AddressOutOfRangeError)
    }

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error> {
        let address = ToIndex::to_index(address).ok_or(AddressOutOfRangeError)?;
        self.0
            .get_mut(address)
            .map(|old| *old = value.into_owned())
            .ok_or(AddressOutOfRangeError)
    }
}
