#[derive(Clone, Debug)]
pub struct Alert {
    pub severity: String,
    pub title: String,
    pub detail: String,
    pub source: String,
    pub time: String,
    pub read: bool,
}
