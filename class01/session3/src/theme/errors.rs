// IKinder

pub fn main() {
    crate::show_name(file!());

    let my_file = std::path::Path::new("my_file.txt");
    let content = std::fs::read_to_string(&my_file);
    match content {
        Ok(c) => println!("{}", c),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found at {:?}", &my_file),
            _ => println!("Error: {e}"),
        },
    }

    if let Ok(content) = file_to_uppercase() {
        println!("Contents: {}", content);
    }
}

fn maybe_read_file() -> Result<String, std::io::Error> {
    let my_file = std::path::Path::new("my_file.txt");
    std::fs::read_to_string(&my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let content = maybe_read_file()?;
    Ok(content.to_uppercase())
}

#[derive(serde::Deserialize)]
struct User {
    user: String,
}

#[derive(Debug, thiserror::Error)]
enum UserError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users found")]
    TooManyUsers,
}

fn load_users() -> Result<Vec<User>, UserError> {
    let my_path = std::path::Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UserError::NoUsers)?;
    let users = serde_json::from_str(&raw_text).map_err(|_| UserError::NoUsers)?;
    // anyhow::bail!("Oh no! We can't do on!");
    Ok(users)
}

fn load_users_anyhow() -> anyhow::Result<Vec<User>> {
    let my_path = std::path::Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path)?;
    let users = serde_json::from_str(&raw_text)?;
    // anyhow::bail!("Oh no! We can't do on!");
    Ok(users)
}
