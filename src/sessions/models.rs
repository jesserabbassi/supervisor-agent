#[derive(Clone, Debug)]
pub struct Session {
    pub customer: String,
    pub pc: String,
    pub start: String,
    pub end: String,
    pub remaining: String,
    pub cost: f32,
    pub status: String,
}
