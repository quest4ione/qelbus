mod array;

pub use self::array::{AddressOutOfRangeError, ArrayMemory, ToIndex};

use nostd_cow::RefCow;

pub trait Memory<T> {
    type Error;

    fn get(&self, address: RefCow<'_, T>) -> Result<RefCow<'_, T>, Self::Error>;

    fn set(&mut self, address: RefCow<'_, T>, value: RefCow<'_, T>) -> Result<(), Self::Error>;
}
