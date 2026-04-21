pub trait Memory<T> {
    type Error;

    fn get(&self, address: &T) -> Result<T, Self::Error>;

    fn modify<O>(&mut self, address: &T, f: impl FnOnce(&mut T) -> O) -> Result<O, Self::Error>;
}
