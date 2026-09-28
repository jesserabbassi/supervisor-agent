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
