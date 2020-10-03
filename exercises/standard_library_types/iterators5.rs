// iterators5.rs

pub fn fibonacci(i: u64) -> u64 {
    // Complete this function to return the ith fibonacci number
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators5` for hints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_of_1() {
        assert_eq!(1, fibonacci(1));
    }
    #[test]
    fn fibonacci_of_2() {
        assert_eq!(2, fibonacci(2));
    }

    #[test]
    fn fibonacci_up_to_10() {
        assert_eq!(
            "[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]",
            format!("{:?}", (0..10).map(fibonacci).collect::<Vec<u64>>())
        );
    }
}
