pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + std::fmt::Debug + Copy,
{
    // V1
    // let mut result = sequence.into_iter().collect::<Vec<_>>();
    // result.dedup();
    // result

    // V2
    // sequence
    //     .into_iter()
    //     .scan(None, |prev, c| {
    //         let keep = *prev != Some(c);
    //         *prev = Some(c);
    //         Some((keep, c))
    //     })
    //     .filter_map(|(keep, c)| keep.then_some(c))
    //     .collect()

    // V3
    let mut unique = sequence.into_iter().collect::<Vec<_>>();
    unique.dedup();
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }
    #[test]
    fn sample_test1() {
        assert_eq!(
            unique_in_order("ABBCcAD".chars()),
            vec!['A', 'B', 'C', 'c', 'A', 'D']
        );
    }
    #[test]
    fn sample_test2() {
        assert_eq!(unique_in_order([1, 2, 2, 3, 3]), vec![1, 2, 3]);
    }
}
