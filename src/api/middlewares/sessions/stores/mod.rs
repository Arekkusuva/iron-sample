pub mod redis;

pub trait SessionStore {
    fn set_raw(&self, key: &str, value: String);
    fn get_raw(&self, key: &str) -> Option<String>;
}
