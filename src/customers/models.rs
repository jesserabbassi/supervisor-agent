#[derive(Clone, Debug)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub membership: String,
    pub balance: f32,
    pub online: bool,
}
