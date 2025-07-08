// IKinder

pub fn main() {
	crate::show_name(file!());
}

async fn double(n: u32) -> u32 {
	n * 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn simple_test() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn will_compile() {
		let rt = tokio::runtime::Builder::new_multi_thread()
			.enable_all()
			.build()
			.unwrap();

		let res = rt.block_on(double(2));
		assert_eq!(res, 4);
	}

	#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
	async fn the_easy_way() {
		assert_eq!(double(2).await, 4);
	}
}
