use std::{collections::HashMap, sync::Arc};

use hyper::{client::Client as HyperClient, Body};
use hyper_tls::HttpsConnector;
use parking_lot::RwLock;
use twilight_gateway::Latency;
use twilight_http::client::Client as HttpClient;
use twilight_model::oauth::Application;

use crate::types::{cache::Cache, context::Context, database::Database};

impl Context {
    pub fn new(
        application: Application,
        cache: Cache,
        database: Database,
        http: HttpClient,
    ) -> Self {
        Self {
            application_id: application.id,
            application_name: application.name,
            cache,
            database,
            http: Arc::new(http),
            hyper: HyperClient::builder().build::<_, Body>(HttpsConnector::new()),
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
