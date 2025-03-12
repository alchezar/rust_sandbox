#[derive(Debug, Eq, PartialEq)]
enum Access {
	Admin,
	User,
	Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
	match name {
		"admin" => Some(Access::Admin),
		"gary" => Some(Access::User),
		_ => None,
	}
}

fn root() -> Option<Access> {
	Some(Access::Admin)
}

fn part_1() -> bool {
	maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
	maybe_access("root").or_else(|| root())
}

fn part_3() -> Access {
	maybe_access("Alice").unwrap_or_else(|| Access::Guest)
}

pub fn run() {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn check_part_1() {
		assert_eq!(part_1(), true, "Admins have an access level");
	}

	#[test]
	fn check_part_2() {
		assert_eq!(part_2(), Some(Access::Admin), "Root users have Admin access")
	}

	#[test]
	fn check_part_3() {
		assert_eq!(part_3(), Access::Guest, "Alice is a guest");
	}
}
