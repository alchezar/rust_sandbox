pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq + std::fmt::Debug + Copy,
{
    sequence
        .into_iter()
        .scan(None, |prev, c| {
            let keep = *prev != Some(c);
            *prev = Some(c);
            Some((keep, c))
        })
        .filter_map(|(keep, c)| keep.then_some(c))
        .collect()

    // let mut result = sequence.into_iter().collect::<Vec<_>>();
    // result.dedup();
    // result
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
