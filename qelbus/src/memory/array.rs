use nostd_cow::RefCow;

use super::Memory;

pub trait ToIndex: Sized {
    fn to_index(address: RefCow<'_, Self>) -> Option<usize>;
}

macro_rules! impl_toindex_prim {
    { $($signed: ty as $unsigned: ty),* $(,)? } => {
        $(
        impl ToIndex for $unsigned {
            fn to_index(address: RefCow<'_, Self>) -> Option<usize> {
                address.into_owned().try_into().ok()
            }
        }

        impl ToIndex for $signed {
            fn to_index(address: RefCow<'_, Self>) -> Option<usize> {
                (address.into_owned() as $unsigned).try_into().ok()
            }
        }
        )*
    }
}

impl_toindex_prim! {
    i8 as u8,
    i16 as u16,
    i32 as u32,
    i64 as u64,
    i128 as u128,
    isize as usize,
}

#[cfg(feature = "num-bigint")]
impl ToIndex for num_bigint::BigUint {
    fn to_index(address: RefCow<'_, Self>) -> Option<usize> {
        (&*address).try_into().ok()
    }
}

#[cfg(feature = "num-bigint")]
impl ToIndex for num_bigint::BigInt {
    fn to_index(address: RefCow<'_, Self>) -> Option<usize> {
        (&*address).try_into().ok()
    }
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
