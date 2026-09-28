#[derive(Clone, Debug)]
pub struct UserSession {
    pub username: String,
    pub role: String,
    pub token: Option<String>,
}
