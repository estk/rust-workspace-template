//! # Example Member
//!
//! This is an example member crate. It exports the add function.
/// Adds two numbers together.
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left.saturating_add(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
