// IKinder

/// # Easy
/// How many unique methods can you use to climb to the top of
/// a staircase with n steps if you are only allowed to climb 1 or 2
/// steps at a time?
///
/// ## Example 1:
/// Input: n = 2\
/// Output: 2\
/// Explanation: To climb to the top of a staircase with 2 steps,
/// you can either climb 1 step twice or climb 2 steps at once.
/// Thus, there are 2 distinct ways to climb to the top.
///
/// ## Example 2:
/// Input: n = 3\
/// Output: 3\
/// Explanation: To climb to the top of a staircase with 3 steps,
/// you can climb 1 step three times, climb 1 step and then 2
/// steps, or climb 2 steps and then 1 step. Thus, there are 3
/// distinct ways to climb to the top.
///
/// ## Example 3:
/// Input: n = 4\
/// Output: 5\
/// Explanation: To climb to the top of a staircase with 4 steps,
/// here are 5 the options: `1, 1, 1, 1`, `1, 1, 2`, `1, 2, 1`,
/// `2, 1, 1`, `2, 2`. Where each number represents a climb of 1 or
/// 2 steps at a time, and the order matters.
///
/// ```
/// assert_eq!(climb_stairs(5), 8);
/// ```
fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut prev = 1;
    let mut curr = 1;

    for _ in 2..=n {
        let next = curr + prev;
        prev = curr;
        curr = next;
    }

    curr
}

/// Pascal's Triangle is a triangular array of numbers where
/// each number is the sum of the two numbers directly above it.
///
/// The first row contains the number 1, and each subsequent row
/// begins and ends with 1. Pascal's Triangle has many
/// mathematical and combinatorial applications, including
/// calculating binomial coefficients and probabilities.
///
/// Pascal's Triangle of size 5,
/// ```
///     1
///    1 1
///   1 2 1
///  1 3 3 1
/// 1 4 6 4 1
/// ```
///
/// # Medium
/// Write a function `pascal` that gets an integer `n` and outputs a
/// **string** which represents two-dimensional array pascal's
/// triangle of size `n`.
///
/// ## Examples
/// Input: `5`\
/// Output: `[[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]`
///
/// Input: `3`\
/// Output: `[[1],[1,1],[1,2,1]]`
/// ```
/// assert_eq!(pascal(1), "[[1]]".to_owned());
/// ```
fn pascal(n: i32) -> String {
    /*	/ using `enumerate` and `fold`
        let vec_to_string = |vec: &[i32]| {
            vec.iter()
                .enumerate()
                .fold(String::from("["), |mut string, (index, &number)| {
                    if index > 0 {
                        string.push(',');
                    }
                    string.push_str(&number.to_string());
                    string
                }) + "]"
        };
    */

    let mut curr = vec![1];
    let mut result = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        curr = (0..=i)
            .map(|j| match (j, i) {
                (0, _) => 1,
                (j, i) if j == i => 1,
                (j, _) => curr[j - 1] + curr[j],
            })
            .collect::<Vec<_>>();

        result.push(format!(
            "[{}]",
            curr.iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    format!("[{}]", result.join(","))
}

/// # Medium
/// Given an integer `n`, return just the `nth` row of the **Pascal's
/// triangle**. You should treat the triangle as 0-indexed.
///
/// # Examples:
/// Input: `0`\
/// Expected output: `[1]`
///
/// Input: `1`\
/// Expected Output: `[1, 1]`
///
/// Input: `2`\
/// Expected Output: `[1, 2, 1]`
/// ```
/// assert_eq!(get_pascal_row(3), Vec::from([1, 3, 3, 1]));
/// ```
fn get_pascal_row(n: i32) -> Vec<i32> {
    let mut row = Vec::new();
    for i in 0..=n as usize {
        row = (0..=i)
            .map(|j| match j {
                0 => 1,
                j if j == i => 1,
                _ => row[j - 1] + row[j],
            })
            .collect::<Vec<_>>();
    }
    row
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn climb_stairs_1() {
        let data = (3, 3);
        assert_eq!(climb_stairs(data.0), data.1);
    }
    #[test]
    fn climb_stairs_2() {
        let data = (5, 8);
        assert_eq!(climb_stairs(data.0), data.1);
    }
    #[test]
    fn climb_stairs_3() {
        let data = (13, 377);
        assert_eq!(climb_stairs(data.0), data.1);
    }
    #[test]
    fn climb_stairs_4() {
        let data = (9, 55);
        assert_eq!(climb_stairs(data.0), data.1);
    }
    #[test]
    fn climb_stairs_5() {
        let data = (1, 1);
        assert_eq!(climb_stairs(data.0), data.1);
    }
    #[test]
    fn pascal_1() {
        let data = (5, "[[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]".to_owned());
        assert_eq!(pascal(data.0), data.1);
    }
    #[test]
    fn pascal_2() {
        let data = (0, "[]");
        assert_eq!(pascal(data.0), data.1);
    }
    #[test]
    fn pascal_3() {
        let data = (1, "[[1]]");
        assert_eq!(pascal(data.0), data.1);
    }
    #[test]
    fn pascal_4() {
        let data = (
            8,
            "[[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1],[1,5,10,10,5,1],[1,6,15,20,15,6,1],[1,7,21,35,35,21,7,1]]",
        );
        assert_eq!(pascal(data.0), data.1);
    }
    #[test]
    fn pascal_5() {
        let data = (
            13,
            "[[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1],[1,5,10,10,5,1],[1,6,15,20,15,6,1],[1,7,21,35,35,21,7,1],[1,8,28,56,70,56,28,8,1],[1,9,36,84,126,126,84,36,9,1],[1,10,45,120,210,252,210,120,45,10,1],[1,11,55,165,330,462,462,330,165,55,11,1],[1,12,66,220,495,792,924,792,495,220,66,12,1]]",
        );
        assert_eq!(pascal(data.0), data.1);
    }
    #[test]
    fn get_pascal_row_1() {
        let data = (3, Vec::from([1, 3, 3, 1]));
        assert_eq!(get_pascal_row(data.0), data.1);
    }
    #[test]
    fn get_pascal_row_2() {
        let data = (5, Vec::from([1, 5, 10, 10, 5, 1]));
        assert_eq!(get_pascal_row(data.0), data.1);
    }
    #[test]
    fn get_pascal_row_3() {
        let data = (
            12,
            Vec::from([1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1]),
        );
        assert_eq!(get_pascal_row(data.0), data.1);
    }
    #[test]
    fn get_pascal_row_4() {
        let data = (
            32,
            Vec::from([
                1, 32, 496, 4960, 35960, 201376, 906192, 3365856, 10518300, 28048800, 64512240,
                129024480, 225792840, 347373600, 471435600, 565722720, 601080390, 565722720,
                471435600, 347373600, 225792840, 129024480, 64512240, 28048800, 10518300, 3365856,
                906192, 201376, 35960, 4960, 496, 32, 1,
            ]),
        );
        assert_eq!(get_pascal_row(data.0), data.1);
    }
}
