// IKinder

#![allow(clippy::manual_pattern_char_comparison)]

/// # K-Pair (warm-up)
/// ## Easy
/// Write a function which gets an array of integers `arr` and a
/// target number `k` and returns `true` if there are a pair of
/// integers in `arr` that sums up to `k`, otherwise returns `false`.
fn find_pair(arr: Vec<i32>, k: i32) -> bool {
    for (i, &value) in arr.iter().enumerate() {
        if arr[(i + 1)..].iter().any(|&next| value + next == k) {
            return true;
        }
    }
    false
}

/// # K-Pair (follow-up)
/// ## Medium
/// The same question from before, but now with time complexity
/// constraint - you must solve it in **O(n)**.
fn find_pair_optimal(arr: Vec<i32>, k: i32) -> bool {
    let mut seen = std::collections::HashSet::new();
    for value in arr {
        if seen.contains(&(k - value)) {
            return true;
        }
        seen.insert(value);
    }
    false
}

/// # K,L Domino
/// ## Hard
/// Write a function `findDominoPair` which gets an array of
/// "dominos" `arr` and a target number `k` and a target number `l`.\
/// Domino is a **string** which represent a pair of 2 integers, in the
/// following format: `"(3;5)"`\
/// The function should return `true` if two dominos `"(ai;bi)"`
/// and `"(aj;bj)"` exists in `arr` and `ai + aj = k` and `bi + bj = l`,
/// otherwise return false.
fn find_domino_pair(arr: Vec<&str>, k: i32, l: i32) -> bool {
    let mut seen = std::collections::HashSet::new();

    for domino in arr {
        let domino = domino
            .trim_matches(|c| c == '(' || c == ')')
            .split(|c| c == ';')
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        let target = 10 * k + l - domino;
        if seen.contains(&target) {
            return true;
        }
        seen.insert(domino);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair_1() {
        let data = (vec![1, 2, 3, 4], 6, true);
        assert_eq!(find_pair(data.0, data.1), data.2);
    }
    #[test]
    fn find_pair_2() {
        let data = (vec![1, 2, 3, 4], 2, false);
        assert_eq!(find_pair(data.0, data.1), data.2);
    }
    #[test]
    fn find_pair_3() {
        let data = (vec![3, 3, 3, 1], 6, true);
        assert_eq!(find_pair(data.0, data.1), data.2);
    }
    #[test]
    fn find_pair_4() {
        let data = (vec![3, 3, 3, 1], 3, false);
        assert_eq!(find_pair(data.0, data.1), data.2);
    }

    #[test]
    fn find_pair_optimal_1() {
        let data = (vec![1, 2, 3, 4], 6, true);
        assert_eq!(find_pair_optimal(data.0, data.1), data.2);
    }
    #[test]
    fn find_pair_optimal_2() {
        let data = (vec![1, 2, 3, 4], 2, false);
        assert_eq!(find_pair_optimal(data.0, data.1), data.2);
    }
    #[test]
    fn find_pair_optimal_3() {
        let data = (vec![3, 3, 3, 1], 6, true);
        assert_eq!(find_pair_optimal(data.0, data.1), data.2);
    }
    #[test]
    fn find_pair_optimal_4() {
        let data = (vec![3, 3, 3, 1], 3, false);
        assert_eq!(find_pair_optimal(data.0, data.1), data.2);
    }

    #[test]
    fn find_domino_pair_1() {
        let data = (vec!["(1;3)", "(2;4)", "(3;1)", "(2;2)"], 3, 5, true);
        assert_eq!(find_domino_pair(data.0, data.1, data.2), data.3);
    }
    #[test]
    fn find_domino_pair_2() {
        let data = (vec!["(1;3)", "(2;4)", "(3;1)", "(2;2)"], 5, 4, false);
        assert_eq!(find_domino_pair(data.0, data.1, data.2), data.3);
    }
    #[test]
    fn find_domino_pair_3() {
        let data = (vec!["(1;3)", "(2;4)", "(3;1)", "(2;2)"], 5, 5, true);
        assert_eq!(find_domino_pair(data.0, data.1, data.2), data.3);
    }
    #[test]
    fn find_domino_pair_4() {
        let data = (
            vec![
                "(1;1)", "(1;1)", "(1;1)", "(1;1)", "(1;1)", "(1;1)", "(1;1)", "(1;1)", "(1;1)",
                "(5;8)", "(1;1)", "(1;1)", "(1;1)", "(1;1)", "(2;3)", "(1;1)", "(1;1)", "(1;1)",
            ],
            7,
            11,
            true,
        );
        assert_eq!(find_domino_pair(data.0, data.1, data.2), data.3);
    }
}
