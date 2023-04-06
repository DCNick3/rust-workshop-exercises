// TASK:
// 1. Implement the increase_volume and decrease_volume functions so that they increase or decrease the volume by 5%, passing the tests.
// 2. Implement the main function

fn increase_volume(volume: i32) -> i32 {
    todo!("Increase the volume by 5%")
}

fn decrease_volume(volume: i32) -> i32 {
    todo!("Decrease the volume by 5%")
}

fn main() {
    let term = console::Term::stdout();

    // this is how you read a single character from the terminal
    let c = term.read_char().expect("Failed to read input");
    // this is how you print formatted text to the terminal
    println!("You pressed: {}", c);

    todo!("Loop indefinitely, read a character specifying whether the volume should be increased or decreased and print the new volume");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_volume_test() {
        assert_eq!(increase_volume(50), 55);
    }

    #[test]
    fn decrease_volume_test() {
        assert_eq!(decrease_volume(50), 45);
    }

    #[test]
    fn increase_volume_test_overflow() {
        assert_eq!(increase_volume(100), 100);
    }

    #[test]
    fn decrease_volume_test_underflow() {
        assert_eq!(decrease_volume(0), 0);
    }
}