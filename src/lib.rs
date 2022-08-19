#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

//! A simple, lightweight library to calculate second order sequences, such as the fibonacci sequence

trait IntoNumber {
    fn into_number(self) -> Number;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "big-int")] {
        use num_bigint::BigInt;

        type Number = BigInt;

        impl IntoNumber for i128 {
            fn into_number(self) -> Number {
                use num_bigint::ToBigInt;

                self.to_bigint().unwrap()
            }
        }
    } else {
        type Number = i128;

        impl IntoNumber for i128 {
            fn into_number(self) -> Number {
                self
            }
        }
    }
}

/// A sequence, represented by the two starting values, which are later used to compute further values
#[derive(Debug, Clone)]
pub struct Sequence(pub Number, pub Number);

impl Sequence {
    /// Creates a new sequence with the given starting values
    pub fn new(sequence: impl Into<Self>) -> Self {
        sequence.into()
    }

    /// Returns the fibonacci sequence, represented as a [`Sequence`] struct
    pub fn fibonacci() -> Self {
        Self::new([1, 1])
    }

    /// Calculate the nth term of the sequence
    pub fn calculate(self, n: usize) -> Number {
        let mut numbers = [self.0, self.1];

        for _ in 3..=n {
            let old_x = &numbers[0];
            let old_y = &numbers[1];
            let new_y = old_x + old_y;
            // Removes the need to clone if we add 0
            numbers[0] = old_y + 0;
            numbers[1] = new_y;
        }

        numbers[1].to_owned()
    }
}

impl From<[i128; 2]> for Sequence {
    fn from(array: [i128; 2]) -> Sequence {
        Sequence(array[0].into_number(), array[1].into_number())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_get_500th() {
        let nth500 = BigInt::from_str("139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125").unwrap();

        let fib = Sequence::fibonacci();

        assert_eq!(fib.calculate(500), nth500);
    }
}
