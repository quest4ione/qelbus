use alloc::boxed::Box;
use nostd_cow::RefCow;

use super::{IndexOutOfRangeError, Memory, ToIndex, to_index};

pub struct PagedArrayMemory<const PAGE_COUNT: usize, const PAGE_SIZE: usize, T>(
    [Option<Box<[T; PAGE_SIZE]>>; PAGE_COUNT],
);

impl<const PAGE_COUNT: usize, const PAGE_SIZE: usize, T> Memory<T>
    for PagedArrayMemory<PAGE_COUNT, PAGE_SIZE, T>
where
    T: Clone + Default + ToIndex,
{
    type Error = IndexOutOfRangeError<T>;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error> {
        let index = to_index(&*address)?;
        let page = self
            .0
            .get(index / PAGE_SIZE)
            .ok_or_else(|| IndexOutOfRangeError::from(address.into_owned()))?;
        Ok(match page {
            Some(page) => RefCow::Borrowed(&page[index % PAGE_SIZE]),
            None => RefCow::Owned(T::default()),
        })
    }

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error> {
        let index = to_index(&*address)?;
        let page = self
            .0
            .get_mut(index / PAGE_SIZE)
            .ok_or_else(|| IndexOutOfRangeError::from(address.into_owned()))?
            .get_or_insert_with(|| Box::new(core::array::from_fn(|_| T::default())));
        page[index % PAGE_SIZE] = value.into_owned();

        Ok(())
    }
}
