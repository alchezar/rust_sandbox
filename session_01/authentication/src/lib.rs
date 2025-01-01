pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username: String = username.to_lowercase();

    if username != "admin" && username != "ivan" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "ivan" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
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
        assert_eq!( login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!( login("Admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!( login("ivan", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("admin", "12345678"), Some(LoginAction::Denied));
        assert_eq!(login("new", "user"), None);
    }
}
