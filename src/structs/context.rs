use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use twilight_gateway::Latency;
use twilight_http::client::{Client, InteractionClient};
use twilight_model::oauth::Application;

use crate::types::{cache::Cache, context::Context, database::Database};

impl Context {
    pub fn interaction_client(&self) -> InteractionClient<'_> {
        self.http.interaction(self.application_id)
    }

    pub fn new(
        application: Application,
        cache: Cache,
        database: Database,
        http: Client,
    ) -> Self {
        Self {
            application_id: application.id,
            application_name: application.name,
            cache,
            database,
            http: Arc::new(http),
            latencies: RwLock::new(HashMap::new()),
        }
    }

    pub fn latency(
        &self,
        shard_id: u64,
    ) -> Option<Arc<Latency>> {
        self.latencies.read().get(&shard_id).cloned()
    }
}
