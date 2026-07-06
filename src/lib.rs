//! Library crate root.

/// Adds two numbers and returns the sum.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `add` returns the sum of its inputs.
    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
    }
}
