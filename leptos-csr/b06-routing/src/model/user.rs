#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub name: &'static str,
    pub role: &'static str,
}