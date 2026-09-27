//! Desktop-owned integration boundary. Backend routes and hub messages are
//! deliberately configurable because the server team owns their contracts.

#[derive(Clone)]
pub struct ApiClient {
    pub base_url: String,
}
impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
    pub fn get(&self, path: &str) -> Result<String, String> {
        Err(format!(
            "Pending API contract: GET {}{}",
            self.base_url, path
        ))
    }
    pub fn post(&self, path: &str, _body: &str) -> Result<String, String> {
        Err(format!(
            "Pending API contract: POST {}{}",
            self.base_url, path
        ))
    }
    pub fn put(&self, path: &str, _body: &str) -> Result<String, String> {
        Err(format!(
            "Pending API contract: PUT {}{}",
            self.base_url, path
        ))
    }
    pub fn delete(&self, path: &str) -> Result<String, String> {
        Err(format!(
            "Pending API contract: DELETE {}{}",
            self.base_url, path
        ))
    }
}

#[derive(Clone)]
pub struct SignalRClient {
    pub hub_url: String,
}
impl SignalRClient {
    pub fn new(hub_url: impl Into<String>) -> Self {
        Self {
            hub_url: hub_url.into(),
        }
    }
    pub fn connect(&self) -> Result<(), String> {
        Err(format!("Pending SignalR contract: {}", self.hub_url))
    }
    pub fn disconnect(&self) {}
    pub fn subscribe(&self, _event: &str) -> Result<(), String> {
        Err("Pending SignalR contract".into())
    }
    pub fn send(&self, _event: &str, _payload: &str) -> Result<(), String> {
        Err("Pending SignalR contract".into())
    }
}

#[derive(Clone, Default)]
pub struct LocalStorage;
impl LocalStorage {
    pub fn save(&self, _key: &str, _value: &str) {}
    pub fn load(&self, _key: &str) -> Option<String> {
        None
    }
    pub fn clear(&self, _key: &str) {}
}
