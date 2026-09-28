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
