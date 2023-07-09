use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use twilight_gateway::Latency;
use twilight_http::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};

use super::{cache::Cache, database::Database};

pub struct Context {
    pub application_id: Id<ApplicationMarker>,
    pub application_name: String,
    pub cache: Cache,
    pub database: Database,
    pub http: Arc<Client>,
    pub latencies: RwLock<HashMap<u64, Arc<Latency>>>,
}
