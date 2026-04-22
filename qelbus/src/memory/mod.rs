#[cfg(feature = "std")]
mod hashmap;

#[cfg(feature = "std")]
pub use self::hashmap::HashMapMemory;

pub trait Memory<T> {
    type Error;

    fn get(&self, address: &T) -> Result<T, Self::Error>;

    fn set(&mut self, address: T, value: T) -> Result<(), Self::Error>;
}
