// This is the primary trait to use when generating random values
use rand::Rng;

pub struct User {
    username: String,
    password_hash: u64,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(password),
        }
    }

    // A getter takes in an immutable reference to its struct object and returns an immutable pointer to its field
    pub fn get_username(&self) -> &String {
        &self.username
    }

    // A setter takes in a mutable reference to its struct object, mutates it with the new value, and returns nothing
    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(new_password)
    }

    pub fn to_string(&self) -> String {
        format!("Username: {}\nPassword: {}", &self.username, &self.password_hash)
    }
}

fn hash_password(input: &str) -> u64 {
    rand::thread_rng().gen()
}