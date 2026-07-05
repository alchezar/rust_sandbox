fn persistence(mut num: u64) -> u64 {
    let mut steps = 0;
    while num >= 10 {
        let mut product = 1;
        while num > 1 {
            product *= num % 10;
            num /= 10;
        }
        num = product;
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(persistence(39), 3);
        assert_eq!(persistence(4), 0);
        assert_eq!(persistence(25), 2);
        assert_eq!(persistence(999), 4);
    }
}
