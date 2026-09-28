#[derive(Clone, Debug)]
pub struct Reservation {
    pub id: String,
    pub customer: String,
    pub pc: String,
    pub time: String,
    pub duration: String,
    pub status: String,
    pub payment: String,
}
