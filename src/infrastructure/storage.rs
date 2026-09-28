#[derive(Clone, Default)]
pub struct LocalStorage;

impl LocalStorage {
    pub fn save(&self, _key: &str, _value: &str) {}
    pub fn load(&self, _key: &str) -> Option<String> {
        None
    }
    pub fn clear(&self, _key: &str) {}
}
