// IKinder

fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_clamp_lower() {
        assert_eq!(clamp(1, 2, 3), 2);
    }

    #[test]
    fn check_clamp_upper() {
        assert_eq!(clamp(4, 2, 3), 3);
    }

    #[test]
    fn check_div_same() {
        assert_eq!(div(1, 1), Some(1));
    }

    #[test]
    fn check_div_zero() {
        assert_eq!(div(1, 1), Some(1));
    }

    #[test]
    fn check_concat() {
        assert_eq!(concat("a", "b"), "ab");
    }
}
