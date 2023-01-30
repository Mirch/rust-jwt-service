#[derive(Copy, Clone)]
pub enum UserRole {
    Admin,
    Regular,
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: UserRole,
}
