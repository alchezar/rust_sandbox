// IKinder

pub fn demo() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut plus_one = vec![];
    for num in &numbers {
        plus_one.push(num + 1);
    }
    println!("plus_one: \t{:?}", plus_one);

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    println!("plus_one iter: \t{:?}", plus_one);

    let new_numbers: Vec<_> = numbers.iter().filter(|num| **num <= 3).collect();
    println!("new_numbers: \t{:?}", new_numbers);

    let find_me = numbers
        .iter()
        .find(|num| **num == 666)
        .unwrap_or_else(|| &&-1);
    println!("find_me: \t{:?}", find_me);

    let count = numbers.iter().count();
    println!("count: \t{:?}", count);

    let last = plus_one.iter().last().unwrap_or_else(|| &-1);
    println!("last: \t{:?}", last);

    let min = numbers.iter().min().unwrap_or_else(|| &&-1);
    println!("min: \t{:?}", min);

    let max = numbers.iter().max().unwrap_or_else(|| &&-1);
    println!("max: \t{:?}", max);

    let take: Vec<_> = numbers.iter().take(4).collect();
    println!("take: \t{:?}", take);
}

pub fn activity() {
    let data = vec![1, 2, 3, 4, 5];
    data.iter()
        .map(|num| num * &3)
        .filter(|num| num > &10)
        .for_each(|num| println!("{:?}", num));
}
