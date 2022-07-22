// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(12, 10, 20), 12, "Should be 12");
        assert_eq!(clamp(8, 10, 20), 10, "Should be 10");
    }

    #[test]
    fn test_div() {
        assert_eq!(div(9, 3), Some(3), "9 / 3 = 3");
        assert_eq!(div(1200, 12), Some(100), "1200 / 12 = 100");
    }

    #[test]
    fn test_concat() {
        assert_eq!(
            concat("hello", "there"),
            String::from("hellothere"),
            "should concat 'hello' and 'there'"
        );
        assert_eq!(
            concat("why, hello", "there"),
            String::from("why, hellothere"),
            "should concat 'why, hello' and 'there'"
        );
    }
}
