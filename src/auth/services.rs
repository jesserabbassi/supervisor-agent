use super::models::UserSession;
use crate::infrastructure::ApiClient;

#[derive(Clone)]
pub struct AuthService {
    pub api: ApiClient,
}

impl AuthService {
    pub fn new(api: ApiClient) -> Self {
        Self { api }
    }

    pub fn current_session(&self) -> UserSession {
        UserSession {
            username: "Admin".into(),
            role: "Supervisor".into(),
            token: None,
        }
    }
}
