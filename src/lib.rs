pub struct Number(f32);

pub const ERROR_BITS: u32 = 4;
pub const ERROR_MAX_VALUE: u32 = (1 << ERROR_BITS) - 1;

const ERROR_MASK: u32 = ERROR_MAX_VALUE; // assuming mantissa is in the least significant bits, which it is
const VALUE_MASK: u32 = !ERROR_MASK;

impl Number {
    /// The meaning of `error` is defined by the user
    pub fn new(value: f32, error: u32) -> Number {
        assert!(error <= ERROR_MAX_VALUE);

        let encoded =
            (value.to_bits() & VALUE_MASK) ^
            (error & ERROR_MASK);

        Number(f32::from_bits(encoded))
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn precise_value(&self) -> f32 {
        f32::from_bits(self.0.to_bits() & VALUE_MASK)
    }

    pub fn error(&self) -> u32 {
        self.0.to_bits() & ERROR_MASK
    }
}

#[cfg(test)]
mod tests {
    use crate::ERROR_MAX_VALUE;
    use crate::Number;

    #[test]
    fn it_works() {
        let n = Number::new(1.0, 2);
        assert_eq!(n.precise_value(), 1.0);
        assert_eq!(n.error(), 2);

        let value_diff = f32::abs(n.value() - 1.0);
        let max_error = f32::abs(n.value() * (n.error() as f32 / ERROR_MAX_VALUE as f32));
        assert!(value_diff < max_error);
    }
}
