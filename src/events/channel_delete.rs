use std::sync::Arc;

use twilight_model::gateway::payload::incoming::ChannelDelete;

use crate::types::{context::Context, Result};

pub fn handle_channel_delete(
    context: Arc<Context>,
    payload: ChannelDelete,
) -> Result<()> {
    context.cache.remove_channel(payload.0.id);

    Ok(())
}
