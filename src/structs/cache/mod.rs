use std::collections::{HashMap, HashSet};

use parking_lot::RwLock;

use crate::types::cache::Cache;

impl Cache {
    pub fn new() -> Self {
        Self {
            guilds: RwLock::new(HashMap::new()),
            members: RwLock::new(HashMap::new()),
            roles: RwLock::new(HashMap::new()),
            unavailable_guilds: RwLock::new(HashSet::new()),
            users: RwLock::new(HashMap::new()),
            voice_states: RwLock::new(HashMap::new()),
        }
    }
}
