// IKinder

/// # Rotate Array
/// ## Easy
/// Write a function that, given an integer array and a number
/// `k`, rotates the array to the right by `k` steps, where `k`
/// is non-negative, and returns the rotated array.
fn rotate_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
	let k = k as usize % nums.len();
	if nums.len() <= 1 || k == 0 {
		return nums;
	}

	let mut nums = nums.clone();
	nums.reverse();
	nums[..k].reverse();
	nums[k..].reverse();
	nums
}

/// # Product Array Except Self
/// ## Medium
/// Write a function that, given an array `nums` of n integers
/// where n > 1, return an array output such that `output[i]`
/// is equal to the product of all the elements of `nums`
/// except `nums[i]`.
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
	let len = nums.len();
	let mut result = vec![1; len];

	let mut prefix = 1;
	for i in 0..len {
		result[i] = prefix;
		prefix *= nums[i];
	}

	let mut suffix = 1;
	for i in (0..len).rev() {
		result[i] *= suffix;
		suffix *= nums[i];
	}

	result
}

/// # Maximum Sum Circular Subarray
/// ## Hard
/// Write a function that, given a circular integer array
/// (i.e., the next element of the last element is the
/// first element of the array), returns the sum of the
/// subarray (containing at least one number) that has the
/// maximum sum.
fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
	use std::cmp::{max, min};

	let (_, max_sum, _, min_sum) = nums.iter().skip(1).fold(
		(nums[0], nums[0], nums[0], nums[0]),
		|(cur_max, max_sum, cur_min, min_sum), &num| {
			let cur_max = max(num, cur_max + num);
			let max_sum = max(max_sum, cur_max);
			let cur_min = min(num, cur_min + num);
			let min_sum = min(min_sum, cur_min);
			(cur_max, max_sum, cur_min, min_sum)
		},
	);

	if max_sum < 0 {
		return max_sum;
	}

	let total_sum = nums.iter().sum::<i32>();
	max(max_sum, total_sum - min_sum)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rotate_array_1() {
		let data = (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_2() {
		let data = (vec![1, 2, 3, 4, 5], 0, vec![1, 2, 3, 4, 5]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_3() {
		let data = (vec![1, 2, 3, 4, 5], 5, vec![1, 2, 3, 4, 5]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_4() {
		let data = (vec![1, 2, 3, 4, 5], 8, vec![3, 4, 5, 1, 2]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_5() {
		let data = (vec![1], 3, vec![1]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_6() {
		let data = (vec![2, 1], 3, vec![1, 2]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_7() {
		let data = (vec![-1, -2, -3, -4], 2, vec![-3, -4, -1, -2]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}
	#[test]
	fn rotate_array_8() {
		let data = (vec![4, 3, 2, 1], 99, vec![3, 2, 1, 4]);
		assert_eq!(rotate_array(data.0, data.1), data.2);
	}

	#[test]
	fn product_except_self_1() {
		let data = (vec![1, 2, 3, 4], vec![24, 12, 8, 6]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_2() {
		let data = (vec![1, 1, 1, 1], vec![1, 1, 1, 1]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_3() {
		let data = (vec![1, 2, 0, 4], vec![0, 0, 8, 0]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_4() {
		let data = (vec![0, 0, 2, 3], vec![0, 0, 0, 0]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_5() {
		let data = (vec![-1, -2, -3, -4], vec![-24, -12, -8, -6]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_6() {
		let data = (vec![1, -2, 3, -4], vec![24, -12, 8, -6]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_7() {
		let data = (vec![5], vec![1]);
		assert_eq!(product_except_self(data.0), data.1);
	}
	#[test]
	fn product_except_self_8() {
		let data = (vec![3, 2], vec![2, 3]);
		assert_eq!(product_except_self(data.0), data.1);
	}

	#[test]
	fn max_subarray_sum_circular_1() {
		let data = (vec![5, -3, 5], 10);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
	#[test]
	fn max_subarray_sum_circular_2() {
		let data = (vec![-2, -3, -1], -1);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
	#[test]
	fn max_subarray_sum_circular_3() {
		let data = (vec![1, 2, 3, 4], 10);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
	#[test]
	fn max_subarray_sum_circular_4() {
		let data = (vec![8, -4, 3, 5, -6], 12);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
	#[test]
	fn max_subarray_sum_circular_5() {
		let data = (vec![10], 10);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
	#[test]
	fn max_subarray_sum_circular_6() {
		let data = (vec![-1, 10], 10);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
	#[test]
	fn max_subarray_sum_circular_7() {
		let data = (vec![0, 0, 0, 3, 0], 3);
		assert_eq!(max_subarray_sum_circular(data.0), data.1);
	}
}
