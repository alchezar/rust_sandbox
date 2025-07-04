use authentication::{get_users, hash_password, save_users, LoginRole, User};
use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[command()]
struct Args {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// List all users.
	List,
	/// Add a user.
	Add {
		/// The user's login name.
		username: String,
		/// The user's password (plain text).
		password: String,
		/// Optional - mark as an admin.
		#[arg(long)]
		admin: Option<bool>,
	},
	/// Delete a user.
	Delete {
		/// User to delete.
		username: String,
	},
	/// Change password.
	ChangePassword {
		/// User which password to change
		username: String,
		/// Old password.
		old_password: String,
		/// New password.
		new_password: String,
	},
}

fn list_users() {
	println!("{:<20}{:<20?}", "Username", "Role");
	println!("{:-<40}", "");

	let users: HashMap<String, User> = get_users();
	users.iter().for_each(|(_, user)| {
		println!("{:<20}{:<20?}", user.username, user.role);
	});
}

fn add_user(username: String, password: String, admin: bool) {
	let mut users: HashMap<String, User> = get_users();
	let role = if admin { LoginRole::Admin } else { LoginRole::User };

	let user: User = User::new(&username, &password, role);
	users.insert(username, user);
	save_users(users);
}

fn delete_user(username: String) {
	let mut users: HashMap<String, User> = get_users();
	if users.contains_key(&username) {
		users.remove(&username);
		save_users(users);
	} else {
		println!("{username} doesn't exist.");
	}
}

fn change_password(username: String, old_password: String, new_password: String) {
	let mut users: HashMap<String, User> = get_users();
	if !users.contains_key(&username) {
		println!("{username} doesn't exist.")
	}
	if users[&username].password != hash_password(old_password.trim()) {
		println!("{username}'s password doesn't match.")
	}
	if let Some(user) = users.get_mut(&username) {
		user.password = hash_password(new_password.trim());
		save_users(users);
	}
}

fn main() {
	let cli: Args = Args::parse();
	match cli.command {
		Some(Commands::List) => {
			list_users();
		}
		Some(Commands::Add { username, password, admin }) => {
			add_user(username, password, admin.unwrap_or(false));
		}
		Some(Commands::Delete { username }) => {
			delete_user(username);
		}
		Some(Commands::ChangePassword { username, old_password, new_password }) => {
			change_password(username, old_password, new_password);
		}
		None => {
			println!("Run with --help to see instructions.");
		}
	}
}
