mod array;
#[cfg(feature = "std")]
mod hashmap;

pub use self::array::{AddressOutOfRangeError, ArrayMemory, ToIndex};
#[cfg(feature = "std")]
pub use self::hashmap::HashMapMemory;

use nostd_cow::RefCow;

pub trait Memory<T> {
    type Error;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error>;

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error>;
}
