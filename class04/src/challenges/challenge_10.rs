// IKinder

/// # Factorial Calculation
/// ## Easy
/// Write a recursive function named `factorial` to calculate the
/// factorial of a given non-negative integer **n**. The factorial of a
/// non-negative integer **n** is the product of all positive integers
/// less than or equal to **n**.
fn factorial(n: i32) -> i32 {
	if n <= 1 {
		return 1;
	}
	n * factorial(n - 1)
}

/// # Combination Calculation
/// ## Medium
/// Calculate the combination **C(n, k)**, which represents the number
/// of ways to choose **k** items from **n** items without regard to order.
///
/// The combination formula is given by `C(n, k) = n! / (k! * (n - k)!)`.
fn combination(n: i32, k: i32) -> i32 {
	if k == 0 || k == n {
		return 1;
	}
	combination(n - 1, k - 1) + combination(n - 1, k)
}

/// # Pascal's Triangle
/// ## Hard
/// Function to generate Pascal's Triangle up to the **n**th row.
/// Pascal's Triangle is a triangular array of the binomial coefficients.
/// The **n**th row of Pascal's Triangle consists of the values of **C(n, k)** for **k**
/// ranging from **0** to **n**.
fn pascal_triangle(n: i32) -> Vec<Vec<i32>> {
	if n <= 0 {
		return vec![vec![1]];
	}
	let mut result = pascal_triangle(n - 1);

	let prev_row = result.last().unwrap();
	let new_row = std::iter::once(1)
		.chain(prev_row.windows(2).map(|w| w[0] + w[1]))
		.chain(std::iter::once(1))
		.collect::<Vec<_>>();
	result.push(new_row);
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn factorial_1() {
		let data = (0, 1);
		assert_eq!(factorial(data.0), data.1);
	}
	#[test]
	fn factorial_2() {
		let data = (12, 479_001_600);
		assert_eq!(factorial(data.0), data.1);
	}
	#[test]
	fn factorial_3() {
		let data = (10, 3_628_800);
		assert_eq!(factorial(data.0), data.1);
	}
	#[test]
	fn factorial_4() {
		let data = (4, 24);
		assert_eq!(factorial(data.0), data.1);
	}
	#[test]
	fn factorial_5() {
		let data = (2, 2);
		assert_eq!(factorial(data.0), data.1);
	}
	#[test]
	fn factorial_6() {
		let data = (3, 6);
		assert_eq!(factorial(data.0), data.1);
	}

	#[test]
	fn combination_1() {
		let data = (5, 2, 10);
		assert_eq!(combination(data.0, data.1), data.2);
	}
	#[test]
	fn combination_2() {
		let data = (6, 3, 20);
		assert_eq!(combination(data.0, data.1), data.2);
	}
	#[test]
	fn combination_3() {
		let data = (0, 0, 1);
		assert_eq!(combination(data.0, data.1), data.2);
	}
	#[test]
	fn combination_4() {
		let data = (5, 0, 1);
		assert_eq!(combination(data.0, data.1), data.2);
	}
	#[test]
	fn combination_5() {
		let data = (5, 5, 1);
		assert_eq!(combination(data.0, data.1), data.2);
	}
	#[test]
	fn combination_6() {
		let data = (10, 5, 252);
		assert_eq!(combination(data.0, data.1), data.2);
	}
	#[test]
	fn combination_7() {
		let data = (27, 12, 17_383_860);
		assert_eq!(combination(data.0, data.1), data.2);
	}

	#[test]
	fn pascal_triangle_1() {
		let data = (
			5,
			vec![
				vec![1],
				vec![1, 1],
				vec![1, 2, 1],
				vec![1, 3, 3, 1],
				vec![1, 4, 6, 4, 1],
				vec![1, 5, 10, 10, 5, 1],
			],
		);
		assert_eq!(pascal_triangle(data.0), data.1);
	}
	#[test]
	fn pascal_triangle_2() {
		let data = (3, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]);
		assert_eq!(pascal_triangle(data.0), data.1);
	}
	#[test]
	fn pascal_triangle_3() {
		let data = (
			7,
			vec![
				vec![1],
				vec![1, 1],
				vec![1, 2, 1],
				vec![1, 3, 3, 1],
				vec![1, 4, 6, 4, 1],
				vec![1, 5, 10, 10, 5, 1],
				vec![1, 6, 15, 20, 15, 6, 1],
				vec![1, 7, 21, 35, 35, 21, 7, 1],
			],
		);
		assert_eq!(pascal_triangle(data.0), data.1);
	}
	#[test]
	fn pascal_triangle_4() {
		let data = (1, vec![vec![1], vec![1, 1]]);
		assert_eq!(pascal_triangle(data.0), data.1);
	}
	#[test]
	fn pascal_triangle_5() {
		let data = (0, vec![vec![1]]);
		assert_eq!(pascal_triangle(data.0), data.1);
	}
}
