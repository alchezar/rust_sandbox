pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "password"
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("User"), "Hello, User!");
    }

    #[test]
    fn test_login() {
        assert!(login("admin", "password"));
        assert!(login("Admin", "password"));
        assert!(!login("username", "password"));
        assert!(!login("admin", "12345678"));
    }
}
