#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub customer: String,
    pub kind: String,
    pub amount: f32,
    pub method: String,
}
