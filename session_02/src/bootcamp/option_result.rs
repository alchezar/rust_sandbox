// IKinder

pub fn run() {
	println!("Hello, world!");

	let username: Option<String> = get_username(1);
	// match username {
	// 	Some(name) => println!("Username: {}", username),
	// 	None => println!("Username not found"),
	// }

	if let Some(name) = username {
		println!("Username: {}", name);
	}
}

fn get_username(user_id: u32) -> Option<String> {
	let query: String = format!("GET username FROM users WHERE id={user_id}");
	let db_result: Result<String, String> = query_db(query);
	db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
	if query.is_empty() {
		Err(String::from("Query string is empty!"))
	} else {
		Ok(String::from("Kinder"))
	}
}
