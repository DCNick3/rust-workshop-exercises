// TASK:
//
// Write the `seconds_since_midnight` function (including the signature) to pass the tests.
//
// The function should take a tuple of three integers representing hours, minutes and seconds and return the total number of seconds since midnight.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_since_midnight() {
        assert_eq!(seconds_since_midnight((0, 0, 0)), 0);
        assert_eq!(seconds_since_midnight((0, 0, 1)), 1);
        assert_eq!(seconds_since_midnight((0, 1, 0)), 60);
        assert_eq!(seconds_since_midnight((1, 0, 0)), 3600);
        assert_eq!(seconds_since_midnight((1, 1, 1)), 3661);
        assert_eq!(seconds_since_midnight((23, 59, 59)), 86399);
    }
}