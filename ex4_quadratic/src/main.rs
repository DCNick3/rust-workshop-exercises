// TASK:
//
// Implement a program to solve quadratic equations.
//
// You should:
// - fill in the QuadraticRoots enum
// - implement the `solve_quadratic_equation` function to pass the tests
// - implement the `main` function to read the coefficients and print the roots.
//
// Use `read_coefficient` function to read the coefficients from the standard input.

#[derive(Debug, PartialEq)] // magic line to make it possible to compare and print the enum (for the tests)
enum QuadraticRoots {
    // FIXME: implement the enum
}

fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> QuadraticRoots {
    todo!("Implement the function to pass the tests")
}

/// Reads a floating point number from the standard input
fn read_coefficient() -> f64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().parse().expect("Failed to parse number")
}

fn main() {
    todo!("Read the coefficients from the standard input and print the roots")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_quadratic_equation() {
        assert_eq!(solve_quadratic_equation(1.0, 0.0, 0.0), QuadraticRoots::OneRoot(0.0));
        assert_eq!(solve_quadratic_equation(1.0, 0.0, 1.0), QuadraticRoots::NoRoots);
        assert_eq!(solve_quadratic_equation(1.0, 0.0, -1.0), QuadraticRoots::TwoRoots(1.0, -1.0));
        assert_eq!(solve_quadratic_equation(1.0, 2.0, 1.0), QuadraticRoots::OneRoot(-1.0));
        assert_eq!(solve_quadratic_equation(1.0, 2.0, 1.0), QuadraticRoots::OneRoot(-1.0));
        assert_eq!(solve_quadratic_equation(1.0, 2.0, 2.0), QuadraticRoots::NoRoots);
        assert_eq!(solve_quadratic_equation(1.0, -2.0, 1.0), QuadraticRoots::OneRoot(1.0));
        assert_eq!(solve_quadratic_equation(0.0, 1.0, 1.0), QuadraticRoots::NotQuadratic);
    }
}