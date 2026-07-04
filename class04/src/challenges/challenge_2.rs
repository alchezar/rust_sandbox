// IKinder

/// # Longest Repeating Ones
/// ## Easy
/// You are given a signal as a string of zeros and ones, return the
/// length of the longest sequence of ones
/// ## Example
/// **Input**:
/// `0110111001011110000000011`\
/// **Expected output**:
/// `4`
fn get_longest_repeating_ones(s: String) -> usize {
    s.chars()
        .fold((0, 0), |(count, largest), c| {
            if c == '1' {
                let new_count = count + 1;
                (new_count, std::cmp::max(largest, new_count))
            } else {
                (0, largest)
            }
        })
        .1
}

/// # With Error Correction
/// ## Medium
/// You are given a **signal** as a string of zeros and ones.
/// Due to some electromagnetic noise, sometimes (although rarely)
/// **you are getting 0 instead of 1**.\
/// Assuming that each signal of 1's may have at most one error bit
/// (meaning 0 instead of 1), return the length of the longest
/// sequence that could possibly be sent to you **with one error only**.
fn get_longest_repeating_ones_with_error(s: String) -> usize {
    let bits = s.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>();
    let zero_indices = bits
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    if zero_indices.is_empty() {
        return bits.len();
    }
    zero_indices
        .iter()
        .map(|&i| {
            let left_count = bits[..i].iter().rev().take_while(|&&b| b == 1).count();
            let right_count = bits[i + 1..].iter().take_while(|&&b| b == 1).count();
            left_count + right_count + 1
        })
        .max()
        .unwrap_or(0)
}

/// # With Double Error Correction
/// ## Hard
/// You are given a `signal` as a string of zeros and ones.
/// Due to some harsh electromagnetic noise, sometimes (not so
/// rarely) you are getting 0 instead of 1.\
/// Assuming that each signal of 1's may have at most two error
/// bits in it (meaning two 0's instead of 1's), return the length
/// of the longest sequence that could possibly be sent to you `with
/// two errors only`.
fn get_longest_repeating_ones_with_errors(s: String) -> usize {
    let chars = s.chars().collect::<Vec<_>>();
    (0..chars.len())
        .map(|start| {
            let mut tolerance = 2;
            chars
                .iter()
                .skip(start)
                .take_while(|&&c| {
                    if c == '0' {
                        if tolerance == 0 {
                            return false;
                        }
                        tolerance -= 1;
                    }
                    true
                })
                .count()
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_longest_repeating_ones_1() {
        let data = ("0110111001011110000000011".to_owned(), 4);
        assert_eq!(get_longest_repeating_ones(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_2() {
        let data = ("000110001110001010101111100".to_owned(), 5);
        assert_eq!(get_longest_repeating_ones(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_3() {
        let data = ("00".to_owned(), 0);
        assert_eq!(get_longest_repeating_ones(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_4() {
        let data = ("111111".to_owned(), 6);
        assert_eq!(get_longest_repeating_ones(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_5() {
        let data = ("0101".to_owned(), 1);
        assert_eq!(get_longest_repeating_ones(data.0), data.1);
    }

    #[test]
    fn get_longest_repeating_ones_with_error_1() {
        let data = ("111100111011101110".to_owned(), 7);
        assert_eq!(get_longest_repeating_ones_with_error(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_with_error_2() {
        let data = ("0011110010110".to_owned(), 5);
        assert_eq!(get_longest_repeating_ones_with_error(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_with_error_3() {
        let data = ("1".to_owned(), 1);
        assert_eq!(get_longest_repeating_ones_with_error(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_with_error_4() {
        let data = ("10".to_owned(), 2);
        assert_eq!(get_longest_repeating_ones_with_error(data.0), data.1);
    }

    #[test]
    fn get_longest_repeating_ones_with_errors_1() {
        let data = ("111100111011101110".to_owned(), 11);
        assert_eq!(get_longest_repeating_ones_with_errors(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_with_errors_2() {
        let data = ("0011110010110".to_owned(), 7);
        assert_eq!(get_longest_repeating_ones_with_errors(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_with_errors_3() {
        let data = ("00".to_owned(), 2);
        assert_eq!(get_longest_repeating_ones_with_errors(data.0), data.1);
    }
    #[test]
    fn get_longest_repeating_ones_with_errors_4() {
        let data = ("1111".to_owned(), 4);
        assert_eq!(get_longest_repeating_ones_with_errors(data.0), data.1);
    }
}
