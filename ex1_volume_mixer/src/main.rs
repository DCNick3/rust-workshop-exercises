fn increase_volume(volume: i32) -> i32 {
    todo!("Increase the volume by 5%")
}

fn decrease_volume(volume: i32) -> i32 {
    todo!("Decrease the volume by 5%")
}

fn main() {
    let term = console::Term::stdout();
    let mut volume = 50;


    loop {
        println!("Current volume: {}%", volume);
        println!("Do you want to increase or decrease the volume? (i/d)");

        match term.read_char().expect("Failed to read input") {
            'i' => volume = increase_volume(volume),
            'd' => volume = decrease_volume(volume),
            _ => println!("Invalid input"),
        }

        println!();
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