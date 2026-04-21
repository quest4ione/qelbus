use core::ops::{AddAssign, SubAssign};

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
    T: Clone + One + SubAssign + Signed,
    for<'a> T: AddAssign<&'a T>,
    M: Memory<T>,
{
    pub fn step(&mut self) -> Result<(), M::Error> {
        let one = T::one();
        let a = self.memory.get(&self.program_counter)?;
        self.program_counter += &one;
        let b = self.memory.get(&self.program_counter)?;
        self.program_counter += &one;
        let c = self.memory.get(&self.program_counter)?;

        let a_val = self.memory.get(&a)?;
        let is_positive = self.memory.modify(b, |b_val| {
            *b_val -= a_val;
            b_val.is_positive()
        })?;

        if is_positive {
            self.program_counter += &one;
        } else {
            self.program_counter = c;
        }

        Ok(())
    }
}
