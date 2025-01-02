pub fn greet_user(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("ivan", "password", LoginRole::User),
    ]
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username: String = username.to_lowercase();
    let users: [User; 2] = get_users();
    if let Some(user) = users.iter().find(|u: &&User| u.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        }
    } else {
        Some(LoginAction::Denied);
    }
    None
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
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("Admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("ivan", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("admin", "12345678"), Some(LoginAction::Denied));
        assert_eq!(login("new", "user"), None);
    }
}
