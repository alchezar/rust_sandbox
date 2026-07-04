// IKinder

/// # Easy
/// Write a function named isPalindrome to determine if a string
/// is a palindrome.
///
/// A palindrome is a word or phrase that reads the same
/// backward as forward.
///
/// The function should take a string as input and return true if it
/// is a palindrome, and false otherwise.
fn is_palindrome(string: &String) -> bool {
    let cleaned = string
        .trim()
        .to_lowercase()
        .chars()
        .into_iter()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    cleaned.chars().eq(cleaned.chars().rev())
}

/// # Medium
/// Write a function named longestPalindrome to find the longest
/// palindromic substring in a string.
///
/// A palindromic substring is a substring that is also a
/// palindrome.
///
/// The function should take a string as input and return the
/// longest palindromic substring.
///
/// If there are multiple palindromic substrings of the same length,
/// the function should return the first one that appears in the string.
fn longest_palindrome(string: &String) -> String {
    let is_palindrome = |str: &str| {
        str.chars()
            .into_iter()
            .zip(str.chars().rev())
            .all(|(a, b)| a == b)
    };
    if string.len() < 2 || is_palindrome(string) {
        return string.clone();
    }

    let mut longest = String::new();
    let chars = string.chars().collect::<Vec<_>>();
    for i in 0..string.len() {
        for j in i..string.len() {
            let sub = chars[i..=j].iter().collect::<String>();
            if is_palindrome(sub.as_str()) && sub.len() > longest.len() {
                longest = sub;
            }
        }
    }
    longest
}

/// # Hard
/// Write a function named `minPalindromeCuts` to find the
/// minimum number of cuts needed to partition a string into a set
/// of palindromes.
///
/// The function should take a string as input and return the
/// minimum number of cuts needed.
///
/// A cut is a partition of the string into two non-empty
/// substrings.
///
/// ## Example:
/// The string "abccbda" can be partitioned into the
/// set of palindromes {"a", "bccb", "d", "a"} with three cuts
fn min_palindrome_cuts(s: &String) -> usize {
    let is_pal = |slice: &[char]| slice.iter().zip(slice.iter().rev()).all(|(a, b)| a == b);

    let chars = s.chars().collect::<Vec<_>>();
    let n = chars.len();

    // Early return in trivial cases.
    if n <= 1 {
        return 0;
    }
    let mut is_palindrome = vec![vec![false; n]; n];
    // Single character palindromes.
    for i in 0..n {
        is_palindrome[i][i] = true;
    }
    // Two characters palindromes.
    for (i, window) in chars.windows(2).enumerate() {
        if window[0] == window[1] {
            is_palindrome[i][i + 1] = true;
        }
    }
    // Longer palindromes.
    for len in 2..n {
        for i in 0..(n - len) {
            let j = i + len;
            is_palindrome[i][j] = chars[i] == chars[j] && is_palindrome[i + 1][j - 1];
        }
    }
    // If the entire string is a palindrome.
    if is_palindrome[0][n - 1] {
        return 0;
    }
    // Dynamic programming array.
    let mut cuts = vec![usize::MAX - 1; n];
    for end in 0..n {
        // If s[0..=i] is a palindrome, no cuts needed.
        if is_palindrome[0][end] {
            cuts[end] = 0;
            continue;
        }
        // Find minimum cuts.
        cuts[end] = (0..end)
            .filter(|&start| is_palindrome[start + 1][end])
            .map(|start| cuts[start] + 1)
            .min()
            .unwrap_or(end);
    }

    cuts[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_1() {
        let data = ("racecar", true);
        assert_eq!(is_palindrome(&data.0.to_owned()), data.1);
    }
    #[test]
    fn is_palindrome_2() {
        let data = ("hello", false);
        assert_eq!(is_palindrome(&data.0.to_owned()), data.1);
    }
    #[test]
    fn is_palindrome_3() {
        let data = ("A man a plan a canal Panama", true);
        assert_eq!(is_palindrome(&data.0.to_owned()), data.1);
    }
    #[test]
    fn is_palindrome_4() {
        let data = ("Was it a car or a cat I saw?", true);
        assert_eq!(is_palindrome(&data.0.to_owned()), data.1);
    }
    #[test]
    fn is_palindrome_5() {
        let data = ("12321", true);
        assert_eq!(is_palindrome(&data.0.to_owned()), data.1);
    }
    #[test]
    fn longest_palindrome_1() {
        let data = ("racecar", "racecar");
        assert_eq!(longest_palindrome(&data.0.to_owned()), data.1.to_owned());
    }
    #[test]
    fn longest_palindrome_2() {
        let data = ("babad", "bab");
        assert_eq!(longest_palindrome(&data.0.to_owned()), data.1.to_owned());
    }
    #[test]
    fn longest_palindrome_3() {
        let data = ("civic", "civic");
        assert_eq!(longest_palindrome(&data.0.to_owned()), data.1.to_owned());
    }
    #[test]
    fn longest_palindrome_4() {
        let data = ("abc", "a");
        assert_eq!(longest_palindrome(&data.0.to_owned()), data.1.to_owned());
    }
    #[test]
    fn longest_palindrome_5() {
        let data = ("aaa", "aaa");
        assert_eq!(longest_palindrome(&data.0.to_owned()), data.1.to_owned());
    }
    #[test]
    fn longest_palindrome_6() {
        let data = ("abccdefg", "cc");
        assert_eq!(longest_palindrome(&data.0.to_owned()), data.1.to_owned());
    }

    #[test]
    fn min_palindrome_cuts_1() {
        let data = ("abccbda", 3);
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
    #[test]
    fn min_palindrome_cuts_2() {
        let data = ("abcbadefg", 4);
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
    #[test]
    fn min_palindrome_cuts_3() {
        let data = ("abbbccdefg", 6);
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
    #[test]
    fn min_palindrome_cuts_4() {
        let data = ("abcdefg", 6);
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
    #[test]
    fn min_palindrome_cuts_5() {
        let data = ("abccba", 0);
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
    #[test]
    fn min_palindrome_cuts_6() {
        let data = (
            "tqxwhcblyhrjlebwqthnomtxqjysbvivirewcgpmlqzievvbldwkhhkhoryehdyxotcqvksjojnayykreyezpxoywznojblpcabe",
            88,
        );
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
    #[test]
    fn min_palindrome_cuts_7() {
        let data = (
            "ixbzuobhaqiowkzmfndefevrntyyjxfuvuqreurynftiuztykcuvknzmledhelmshdhjenmiogvedsthoxgauwmwlcikfiymojjcvdqxwbgvongpcqxsgvqlqqbbrbcnehoxtnswgdjavsmydyyptxnqyceftyenwhncfyvxvndwcyozzscxcgdielohkklzfbefvaxxklpxzmodlcobhrmfuywxivqxvsoogjgdhjedvbchuuoblqfwlxdqbgdleygbqlirfgbldxvzkmldjehkaervcqotnycbzzvnelwruzfcaauucaollboagslsmfxrkdhatzzautwyxdrqwwtveffkrxjoqesaemspmenhmdphhgzxttmbvaggjvkxynbhioxzxcohvjsvwklnjjuhhayubdmlwstitppzjbsvreziyiayfassqfhkazjdmoichutntotnoyesovwsftkgpojvwxddeteelonifphbtfurxteulolmtqkkqskisqiypavtswiszfoyrzvwyxiswrrczmfiotpogjhmrntlxdmgvbuzdkfzrqbzoaaaokzoppyffdtkdwfnhrwnclmlylergfmxpemtihbbpmtpizgazzqteriuccmmlnaqpdaasgrmoccyojswnugbgevsnzcgyskbbpfqfbgkytjoyptxqkjmvxmvmwoorgoifniiwxmixbgfxrnfkhdwsijjybkkisrtmshsyxxfmjbhqarwwxjmbjoabyxlwjggheseqazwgzqttdsxjdtoqztzfeyskedbtglglwuiliicenbeccgsudfgycehbipghzugvhhwhjffoextomontsivxqbwnyobwejrqqsoavusepimvhhgdimovbosnvirftuywzjkjpuvsqljaxyvxrychvylosjjdhbesvtfydnxffnecjovrtrmjeniaicuvbcnsdtyrjfxsovyfbvcqgfetupegbrtqqsvsukc",
            879,
        );
        assert_eq!(min_palindrome_cuts(&data.0.to_owned()), data.1);
    }
}
