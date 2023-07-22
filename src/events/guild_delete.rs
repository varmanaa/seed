use std::sync::Arc;

use twilight_model::gateway::payload::incoming::GuildDelete;

use crate::types::{context::Context, Result};

pub async fn handle_guild_delete(
    context: Arc<Context>,
    payload: GuildDelete,
) -> Result<()> {
    let guild_id = payload.id;

    context.database.remove_guild(guild_id).await?;
    context.database.remove_guild_levels(guild_id).await?;
    context.cache.remove_guild(guild_id, payload.unavailable);

    Ok(())
}
