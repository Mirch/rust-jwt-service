use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};

use crate::models::{User, UserRole};

pub struct Database {
    pub users: Arc<RwLock<HashMap<String, User>>>,
}

impl Database {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(
            String::from("admin"),
            User {
                username: String::from("admin"),
                password: String::from("admin"),
                role: UserRole::Admin,
            },
        );

        let users = Arc::new(RwLock::new(users));
        Self { users }
    }

    pub fn insert(&self, user: &User) -> Result<()> {
        let mut data = self.users.write().unwrap();
        let data = &mut *data;

        let key = (&user.username).to_string();
        data.insert(key, user.clone());

        Ok(())
    }

    pub fn get(&self, username: &str) -> Option<User> {
        let data = self.users.read().unwrap();
        let data = &*data;

        data.get(username).map(|u| u.clone())
    }
}
