mod channel;
mod guild;
mod member;
mod unavailable_guild;

use std::collections::{HashMap, HashSet};

use parking_lot::RwLock;

use crate::types::cache::Cache;

impl Cache {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            guilds: RwLock::new(HashMap::new()),
            members: RwLock::new(HashMap::new()),
            unavailable_guilds: RwLock::new(HashSet::new()),
        }
    }
}
