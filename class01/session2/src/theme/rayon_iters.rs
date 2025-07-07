// IKinder

use rayon::prelude::*;

pub fn main() {
	crate::show_name(file!());

	let numbers = (0..1_000_000).collect::<Vec<u64>>();
	let sum = numbers.par_iter().sum::<u64>();
	println!("sum: {}", sum);

	rayon_approach();
	default_approach();
}

fn is_prime(n: u32) -> bool {
	(2..=n / 2)
		.into_par_iter()
		.all(|i| n % i != 0)
}

fn default_approach() {
	let start = std::time::Instant::now();
	let numbers = (0..1000).collect::<Vec<u32>>();
	let mut primes = numbers
		.iter()
		.filter(|n| is_prime(**n))
		.collect::<Vec<_>>();
	primes.sort_unstable();
	let elapsed = start.elapsed().as_secs_f32();
	println!("DEFAULT: Found {} primes in {} seconds", primes.len(), elapsed);
}

fn rayon_approach() {
	let start = std::time::Instant::now();
	let numbers = (0..1000).collect::<Vec<u32>>();
	let mut primes = numbers
		.par_iter()
		.filter(|n| is_prime(**n))
		.collect::<Vec<_>>();
	primes.par_sort_unstable();
	let elapsed = start.elapsed().as_secs_f32();
	println!("RAYON: Found {} primes in {} seconds", primes.len(), elapsed);
}
