use nostd_cow::RefCow;

use super::{IndexOutOfRangeError, Memory, ToIndex, to_index};

pub struct ArrayMemory<const N: usize, T>([T; N]);

impl<const N: usize, T> Memory<T> for ArrayMemory<N, T>
where
    T: Clone + ToIndex,
{
    type Error = IndexOutOfRangeError<T>;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error> {
        let index = to_index(&*address)?;
        self.0
            .get(index)
            .map(RefCow::Borrowed)
            .ok_or_else(|| IndexOutOfRangeError::from(address.into_owned()))
    }

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error> {
        let index = to_index(&*address)?;
        self.0
            .get_mut(index)
            .map(|old| *old = value.into_owned())
            .ok_or_else(|| IndexOutOfRangeError::from(address.into_owned()))
    }
}
