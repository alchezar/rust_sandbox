use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn get_default_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("ivan", "password", LoginRole::User),
    ]
}

fn get_users() -> HashMap<String, User> {
    let users_path = std::path::Path::new("users.json");
    if users_path.exists() {
        // load the file!
	    let users_json = std::fs::read_to_string(users_path).unwrap();
	    let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
	    users
    } else {
        // create a file and return it
        let users: HashMap<String, User> = get_hash_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

fn get_hash_users() -> HashMap<String, User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "ivan".to_string(),
        User::new("ivan", "password", LoginRole::Admin),
    );
    users
}

fn get_admin_users() -> Vec<String> {
    get_default_users()
        .into_iter()
        .filter(|u| u.role == LoginRole::Admin)
        .map(|u| u.username)
        .collect()
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username: String = username.to_lowercase();
    let users: HashMap<String, User> = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            Some(LoginAction::Denied);
        }
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
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("Admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("ivan", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "12345678"), Some(LoginAction::Denied));
        assert_eq!(login("new", "user"), None);
    }
}
