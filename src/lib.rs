#![warn(
    clippy::all,
    nonstandard_style,
    future_incompatible,
    missing_debug_implementations
)]
#![forbid(unsafe_code)]

pub mod client;
pub mod error;

/// test
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
