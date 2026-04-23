use core::ops::{AddAssign, SubAssign};

use nostd_cow::RefCow;
use num_traits::{One, Signed};

use crate::Memory;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Computer<T, M> {
    pub program_counter: T,
    pub memory: M,
}

impl<T, M> Computer<T, M>
where
    T: Clone + One + Signed,
    for<'a> T: AddAssign<&'a T> + SubAssign<&'a T>,
    M: Memory<T>,
{
    pub fn step(&mut self) -> Result<(), M::Error> {
        let one = T::one();
        let a = self.memory.get(RefCow::Borrowed(&self.program_counter))?;
        self.program_counter += &one;
        let b = self.memory.get(RefCow::Borrowed(&self.program_counter))?;
        self.program_counter += &one;
        let c = self
            .memory
            .get(RefCow::Borrowed(&self.program_counter))?
            .into_owned();

        let a_val = self.memory.get(a)?;
        let mut b_val = self.memory.get(RefCow::Borrowed(&b))?.into_owned();
        b_val -= &*a_val;
        let is_positive = b_val.is_positive();
        self.memory
            .set(RefCow::Owned(b.into_owned()), RefCow::Owned(b_val))?;

        if is_positive {
            self.program_counter += &one;
        } else {
            self.program_counter = c;
        }

        Ok(())
    }
}
