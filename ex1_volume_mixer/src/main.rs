// TASK:
// 1. Implement the increase_volume and decrease_volume functions so that they increase or decrease the volume by 5%, passing the tests.
// 2. Implement the main function

fn increase_volume(volume: i32) -> i32 {
    let volume = volume + 5;
    if volume > 100 {
        100
    } else {
        volume
    }
}

fn decrease_volume(volume: i32) -> i32 {
    let volume = volume - 5;
    if volume < 0 {
        0
    } else {
        volume
    }
}

fn main() {
    let term = console::Term::stdout();

    let mut volume = 50;

    loop {
        println!("Current volume: {}", volume);
        println!("What should I do (i/d)?");

        let c = term.read_char().unwrap();
        if c == 'i' {
            volume = increase_volume(volume);
        } else if c == 'd' {
            volume = decrease_volume(volume);
        } else {
            println!("Unknown action");
        }
    }
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