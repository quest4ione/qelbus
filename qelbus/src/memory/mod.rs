mod array;

use core::{
    error::Error,
    fmt::{Debug, Display},
};

use nostd_cow::RefCow;

pub use self::array::ArrayMemory;

pub trait Memory<T> {
    type Error;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error>;

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error>;
}

pub trait ToIndex: Sized {
    fn to_index(&self) -> Option<usize>;
}

macro_rules! impl_toindex_prim {
    { $($signed: ty as $unsigned: ty),* $(,)? } => {
        $(
        impl ToIndex for $unsigned {
            fn to_index(&self) -> Option<usize> {
                (*self).try_into().ok()
            }
        }

        impl ToIndex for $signed {
            fn to_index(&self) -> Option<usize> {
                (*self as $unsigned).try_into().ok()
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
    fn to_index(&self) -> Option<usize> {
        self.try_into().ok()
    }
}

#[cfg(feature = "num-bigint")]
impl ToIndex for num_bigint::BigInt {
    fn to_index(&self) -> Option<usize> {
        self.try_into().ok()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct IndexOutOfRangeError<T>(T);

impl<T> From<T> for IndexOutOfRangeError<T> {
    fn from(value: T) -> Self {
        IndexOutOfRangeError(value)
    }
}

impl<T: Display> Display for IndexOutOfRangeError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "index {} out of range", self.0)
    }
}

impl<T: Display + Debug> Error for IndexOutOfRangeError<T> {}

impl<T> IndexOutOfRangeError<T> {
    pub fn index(&self) -> &T {
        &self.0
    }

    pub fn into_index(self) -> T {
        self.0
    }
}

pub fn to_index<T: ToIndex + Clone>(value: &T) -> Result<usize, IndexOutOfRangeError<T>> {
    value.to_index().ok_or_else(|| value.clone().into())
}
