// IKinder

/// # Merge Sort
/// ## Easy
/// Write a function that gets two sorted lists as input, merges
/// them into one sorted list, and returns it.
fn merge_sort(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(a1.len() + a2.len());
    let mut iter1 = a1.iter().peekable();
    let mut iter2 = a2.iter().peekable();

    while let (Some(&&v1), Some(&&v2)) = (iter1.peek(), iter2.peek()) {
        if v1 < v2 {
            result.push(*iter1.next().unwrap());
        } else {
            result.push(*iter2.next().unwrap());
        }
    }

    result.extend(iter1);
    result.extend(iter2);
    result
}

/// # Almost Palindrome
/// ## Medium
/// Write a function that gets a string as an input, returns `true`
/// if the string is almost a palindrome, and returns `false` elsewise.
fn is_almost_palindrome(s: String) -> bool {
    let half = s.len() / 2 - 1;
    let count = s
        .chars()
        .into_iter()
        .take(half)
        .zip(s.chars().into_iter().rev().take(half))
        .filter(|(c1, c2)| *c1 == *c2)
        .count();

    count >= half
}

/// # Second Most Popular
/// ## Hard
/// Write a function that gets a list of integers as an input,
/// and returns the second most recurring integer.
fn second_most(a: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let map = a.iter().fold(HashMap::with_capacity(a.len()), |mut m, &v| {
        m.entry(v).and_modify(|x| *x += 1).or_insert(1);
        m
    });
    let top_two = map.iter().fold([(0, 0); 2], |mut top, (&key, &value)| {
        if value > top[0].1 {
            top[1] = top[0];
            top[0] = (key, value);
        } else if value > top[1].1 {
            top[1] = (key, value)
        }
        top
    });
    top_two[1].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_1() {
        let data = (
            vec![1, 4, 6, 8, 14, 23],
            vec![2, 3, 5, 7, 11, 18, 19, 20],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 11, 14, 18, 19, 20, 23],
        );
        assert_eq!(merge_sort(data.0, data.1), data.2);
    }
    #[test]
    fn merge_sort_2() {
        let data = (vec![2, 3, 4], vec![2, 3, 4], vec![2, 2, 3, 3, 4, 4]);
        assert_eq!(merge_sort(data.0, data.1), data.2);
    }
    #[test]
    fn merge_sort_3() {
        let data = (
            vec![1, 4, 6, 34, 123],
            vec![5, 6, 34],
            vec![1, 4, 5, 6, 6, 34, 34, 123],
        );
        assert_eq!(merge_sort(data.0, data.1), data.2);
    }
    #[test]
    fn merge_sort_4() {
        let data = (vec![1, 2, 3], vec![4, 5, 6], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge_sort(data.0, data.1), data.2);
    }
    #[test]
    fn merge_sort_5() {
        let data = (vec![4, 5, 6], vec![1, 2, 3], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge_sort(data.0, data.1), data.2);
    }

    #[test]
    fn is_almost_palindrome_1() {
        let data = ("racebar".to_owned(), true);
        assert_eq!(is_almost_palindrome(data.0), data.1);
    }
    #[test]
    fn is_almost_palindrome_2() {
        let data = ("racecar".to_owned(), true);
        assert_eq!(is_almost_palindrome(data.0), data.1);
    }
    #[test]
    fn is_almost_palindrome_3() {
        let data = ("abcdcab".to_owned(), false);
        assert_eq!(is_almost_palindrome(data.0), data.1);
    }
    #[test]
    fn is_almost_palindrome_4() {
        let data = ("mok".to_owned(), true);
        assert_eq!(is_almost_palindrome(data.0), data.1);
    }
    #[test]
    fn is_almost_palindrome_5() {
        let data = ("racebaa".to_owned(), false);
        assert_eq!(is_almost_palindrome(data.0), data.1);
    }
    #[test]
    fn is_almost_palindrome_6() {
        let data = ("vxxxxxxxxaxx".to_owned(), false);
        assert_eq!(is_almost_palindrome(data.0), data.1);
    }

    #[test]
    fn second_most_1() {
        let data = (
            vec![98, 72, 93, 72, 83, 98, 72, 44, 25, 53, 43, 98, 81, 98],
            72,
        );
        assert_eq!(second_most(data.0), data.1);
    }
    #[test]
    fn second_most_2() {
        let data = (vec![1, 1, 2, 2, 1, 3], 2);
        assert_eq!(second_most(data.0), data.1);
    }
    #[test]
    fn second_most_3() {
        let data = (vec![2, 5, 5], 2);
        assert_eq!(second_most(data.0), data.1);
    }
    #[test]
    fn second_most_4() {
        let data = (vec![5, 5, 4, 3, 5, 4, 1], 4);
        assert_eq!(second_most(data.0), data.1);
    }
    #[test]
    fn second_most_5() {
        let data = (
            vec![
                70, 35, 13, 50, 29, 36, 38, 87, 45, 53, 23, 6, 32, 41, 28, 5, 10, 41, 40, 61, 28,
                21, 57, 54, 73, 28, 86, 52, 39, 65, 12, 84, 41, 24, 80, 88, 65, 57, 6, 54, 81, 2,
                21, 11, 27, 87, 45, 86, 97, 38, 88, 44, 44, 92, 80, 35, 40, 85, 71, 38, 46, 97, 36,
                3, 62, 70, 75, 53, 49, 2, 39, 64, 68, 10, 82, 68, 8, 41, 10, 44, 72, 2, 46, 8, 53,
                10, 93, 77, 34, 25, 60, 38, 83, 77, 55, 23, 90, 40, 31, 21, 80, 48, 79, 86, 11, 51,
                36, 83, 70, 31, 3, 9, 38, 60, 13, 24, 51, 7, 68, 25, 99, 5, 7, 22, 90, 48, 16, 63,
                69, 43, 1, 97, 89, 46, 41, 68, 50, 31, 85, 84, 68, 30, 59, 78, 36, 14, 29, 75, 85,
                20, 74, 33, 66, 98, 28, 19, 4, 23, 51, 9, 28, 6, 13, 9, 36, 84, 99, 79, 94, 8, 47,
                89, 76, 34, 70, 34, 98, 82, 80, 44, 5, 92, 74, 87, 65, 37, 95, 72, 81, 85, 30, 36,
                39, 52, 23, 53, 18, 74, 84, 4, 88, 57, 0, 77, 13, 40, 36, 0, 41, 15, 21, 61, 11,
                92, 26, 98, 53, 97, 23, 1, 41, 74, 49, 28, 50, 7, 78, 73, 1, 86, 62, 74, 78, 49,
                55, 79, 2, 17, 95, 64, 12, 33, 49, 28, 58, 39, 6, 23, 35, 34, 5, 42, 75, 44, 24,
                65, 3, 6, 77, 14, 57, 31, 10, 28, 23, 40, 85, 53, 0, 87, 95, 33, 29, 80, 5, 55, 72,
                86, 83, 5, 48, 43, 78, 88, 47, 25, 53, 93, 15, 92, 87, 24, 23, 18, 5, 61, 17, 74,
                6, 46, 64, 34, 40, 27, 50, 47, 33, 58, 54, 90, 62, 4, 74, 21, 62, 91, 95, 39, 2,
                86, 43, 99, 40, 97, 7, 13, 35, 46, 62, 50, 48, 22, 68, 18, 24, 68, 70, 13, 34, 57,
                94, 41, 35, 19, 46, 10, 17, 23, 42, 79, 45, 10, 17, 31, 44, 57, 77, 39, 34, 60, 57,
                41, 93, 49, 39, 15, 53, 88, 43, 77, 48, 79, 23, 22, 16, 49, 84, 9, 69, 57, 29, 59,
                68, 8, 32, 89, 20, 45, 91, 73, 62, 29, 80, 20, 37, 62, 98, 20, 19, 40, 99, 34, 12,
                43, 74, 97, 9, 32, 86, 35, 0, 23, 87, 80, 11, 60, 20, 83, 76, 61, 62, 82, 49, 28,
                28, 74, 52, 37, 49, 73, 60, 27, 8, 38, 15, 34, 86, 43, 7, 49, 66, 59, 50, 80, 18,
                67, 39, 56, 35, 8, 42, 54, 61, 54, 68, 79, 23, 27, 53, 85, 3, 78, 10, 18, 57, 32,
                48, 85, 21, 0, 78, 49, 80, 9, 80, 94, 79, 25, 81, 41, 24, 73, 83, 53, 44, 30, 50,
                64, 43, 70, 80, 48, 31, 10, 58, 98, 68, 7, 78, 0, 1, 21, 44, 98, 99, 22, 59, 79,
                55, 1, 21, 87, 27, 91, 92, 34, 3, 98, 0, 0, 60, 11, 57, 15, 8, 46, 79, 87, 82, 97,
                27, 80, 56, 85, 47, 6, 41, 34, 48, 34, 23, 60, 31, 32, 99, 95, 95, 14, 87, 72, 7,
                82, 3, 29, 2, 70, 49, 66, 61, 84, 6, 8, 3, 76, 21, 66, 79, 70, 50, 83, 26, 9, 55,
                68, 7, 97, 18, 76, 12, 97, 84, 35, 62, 46, 85, 95, 66, 81, 17, 93, 44, 12, 0, 42,
                52, 20, 37, 22, 73, 76, 3, 88, 31, 49, 13, 50, 20, 61, 57, 60, 51, 31, 20, 3, 14,
                32, 62, 45, 92, 42, 13, 13, 77, 2, 71, 2, 66, 37, 81, 61, 69, 4, 81, 61, 98, 32, 5,
                78, 17, 47, 81, 44, 44, 36, 91, 16, 24, 93, 31, 89, 80, 39, 24, 58, 81, 80, 2, 1,
                20, 65, 8, 66, 16, 73, 69, 37, 96, 15, 85, 66, 53, 48, 11, 82, 81, 94, 65, 92, 84,
                26, 78, 19, 32, 79, 13, 75, 48, 37, 81, 97, 8, 19, 54, 87, 69, 63, 31, 95, 12, 13,
                41, 55, 42, 61, 30, 61, 9, 57, 87, 63, 10, 58, 50, 16, 16, 99, 87, 65, 34, 84, 63,
                46, 83, 50, 94, 66, 50, 3, 83, 38, 96, 2, 23, 20, 12, 75, 38, 33, 40, 36, 42, 80,
                83, 26, 17, 1, 56, 94, 63, 74, 97, 2, 39, 67, 73, 33, 14, 16, 37, 2, 81, 18, 86,
                21, 15, 30, 82, 76, 76, 18, 32, 51, 84, 5, 95, 89, 24, 10, 39, 19, 91, 71, 23, 68,
                1, 83, 17, 11, 53, 54, 81, 94, 40, 51, 3, 63, 62, 7, 92, 74, 14, 40, 95, 34, 60,
                17, 21, 81, 70, 82, 41, 53, 13, 6, 38, 80, 56, 25, 53, 26, 55, 60, 57, 18, 52, 28,
                66, 97, 86, 22, 12, 56, 50, 90, 88, 17, 76, 4, 22, 78, 22, 67, 95, 51, 60, 73, 37,
                99, 2, 2, 32, 8, 72, 76, 17, 47, 97, 62, 88, 65, 60, 60, 57, 81, 53, 43, 86, 3, 25,
                25, 65, 10, 74, 10, 36, 83, 46, 69, 3, 70, 86, 17, 53, 42, 62, 75, 51, 96, 57, 85,
                84, 83, 4, 70, 1, 47, 41, 54, 19, 96, 11, 74, 2, 16, 19, 53, 39, 5, 44, 69, 87, 34,
                66, 54, 31, 98, 8, 72, 65, 93, 14, 77, 8, 13, 66, 4, 46, 91, 62, 33, 79, 57, 66,
                95, 35, 30, 28, 40, 33, 55, 15, 94, 25, 9, 34, 95, 90, 73, 79, 41, 56, 24, 53, 58,
                78, 77, 56, 2, 53, 60, 72, 94, 46, 6, 10, 41, 8, 35, 80, 97, 22, 61, 94, 95, 7, 89,
                84, 22, 50, 52, 96, 80, 43, 4, 79, 44, 89, 27, 3, 46, 41, 95, 69, 66, 33, 74, 97,
                40, 43, 56,
            ],
            80,
        );
        assert_eq!(second_most(data.0), data.1);
    }
}
