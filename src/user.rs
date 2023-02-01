#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UserRole {
    Admin,
    Regular,
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Admin" => UserRole::Admin,
            "Regular" => UserRole::Regular,
            _ => UserRole::Regular,
        }
    }
}

impl Into<String> for UserRole {
    fn into(self) -> String {
        match self {
            UserRole::Admin => String::from("Admin"),
            UserRole::Regular => String::from("Regular"),
        }
    }
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: UserRole,
}
