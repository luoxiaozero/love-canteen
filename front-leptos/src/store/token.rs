use gloo_storage::{LocalStorage, Storage};

const TOKEN: &str = "token";
pub struct Token;

impl Token {
    pub fn get() -> Option<String> {
        let Ok(token) = LocalStorage::get(TOKEN) else {
            return None;
        };

        token
    }

    pub fn set(token: String) {
        _ = LocalStorage::set(TOKEN, token);
    }
}