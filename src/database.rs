use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::{anyhow, Result};

use crate::models::User;

pub struct Database {
    pub users: Arc<RwLock<HashMap<String, User>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(self, user: &User) -> Result<()> {
        let mut data = self.users.write().unwrap();
        let data = &mut *data;

        let key = (&user.username).to_string();
        data.insert(key, user.clone());

        Ok(())
    }

    pub fn get(self, username: &str) -> Result<User> {
        let data = self.users.read().unwrap();
        let data = &*data;

        match data.get(username) {
            Some(value) => Ok(value.clone()),
            None => Err(anyhow!("Could not find user for username {}", username)),
        }
    }
}
